use std::collections::{HashMap, HashSet};
use zbus::{interface, zvariant::{OwnedObjectPath, Value}, Address, Connection};
use futures::future::join_all;
use tracing::{debug, error, info, warn};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use clap::{Parser, Subcommand};

// --- Command-Line Argument Parsing ---
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run in server mode, listening for D-Bus requests. (For systemd/D-Bus activation)
    Server,
    /// Send a notification to all users.
    Send {
        /// The title of the notification.
        title: String,
        /// The body message of the notification.
        body: String,
    },
}

// --- D-Bus Interface & Proxy Definitions ---
const DBUS_INTERFACE_NAME: &str = "me.section.Notifier";
const DBUS_PATH: &str = "/me/section/Notifier";

// The SERVICE implementation (for the server)
struct NotifierService;

#[interface(name = "dme.section.Notifier")]
impl NotifierService {
    async fn send_to_all(&self, title: String, body: String) -> zbus::fdo::Result<()> {
        info!(%title, %body, "Received 'send_to_all' request via D-Bus.");

        let users = match get_active_graphical_users().await {
            Ok(users) => users,
            Err(e) => {
                error!("Failed to get active users: {}", e);
                return Err(zbus::fdo::Error::Failed(e.to_string()));
            }
        };

        if users.is_empty() {
            warn!("No active graphical user sessions found to notify.");
            return Ok(());
        }

        info!("Dispatching notifications to {} users: {:?}", users.len(), users);

        let notification_tasks = users.into_iter().map(|user| {
            let title_clone = title.clone();
            let body_clone = body.clone();
            tokio::spawn(async move {
                let user_span = tracing::info_span!("user_notification", uid = user.uid, username = %user.username);
                let _enter = user_span.enter();
                if let Err(e) = send_notification_to_user(&user, &title_clone, &body_clone).await {
                    error!("Failed to send notification: {}", e);
                } else {
                    info!("Notification sent successfully.");
                }
            })
        });

        join_all(notification_tasks).await;
        Ok(())
    }
}

// --- Main Application Logic ---
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Server => run_server().await?,
        Commands::Send { title, body } => run_client(&title, &body).await?,
    }

    Ok(())
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting in server mode...");
    let _conn = zbus::connection::Builder::system()?
        .name(DBUS_INTERFACE_NAME)?
        .serve_at(DBUS_PATH, NotifierService)?
        .build()
        .await?;

    info!("Notifier service is up and listening on the system bus.");
    std::future::pending::<()>().await;
    Ok(())
}

// The CLIENT proxy definition
#[zbus::proxy(
    interface = "me.section.Notifier",
    default_service = "me.section.Notifier",
    default_path = "/me/section/Notifier"
)]
trait Notifier {
    async fn send_to_all(&self, title: &str, body: &str) -> zbus::Result<()>;
}

async fn run_client(title: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting in client mode...");
    let connection = Connection::system().await?;
    let proxy = NotifierProxy::new(&connection).await?;

    info!("Sending notification request to the system service...");
    proxy.send_to_all(title, body).await?;
    info!("Request sent successfully.");

    Ok(())
}

// --- Notification & Systemd Logic ---

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct TargetUser {
    uid: u32,
    username: String,
}

#[zbus::proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
trait LoginManager {
    #[zbus(name = "ListSessions")]
    fn list_sessions(&self) -> zbus::Result<Vec<(String, u32, String, String, OwnedObjectPath)>>;
}

#[zbus::proxy(
    interface = "org.freedesktop.login1.Session",
    default_service = "org.freedesktop.login1"
)]
trait Session {
    #[zbus(property)]
    fn active(&self) -> zbus::Result<bool>;

    #[zbus(property, name = "Type")]
    fn session_type(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn user(&self) -> zbus::Result<(u32, OwnedObjectPath)>;
}

#[zbus::proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: &HashMap<&str, Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;
}


async fn get_active_graphical_users() -> Result<HashSet<TargetUser>, Box<dyn std::error::Error>> {
    let mut active_users = HashSet::new();
    let sys_bus = Connection::system().await?;
    let manager_proxy = LoginManagerProxy::new(&sys_bus).await?;
    let sessions = manager_proxy.list_sessions().await?;
    for (session_id, _uid, username, _seat, session_path) in sessions {
        let session_span = tracing::debug_span!("session_check", id = %session_id, user = %username);
        let _enter = session_span.enter();

        // FIX: Use the correct builder pattern for a dynamic path
        let session_proxy = SessionProxy::builder(&sys_bus)
            .path(session_path)?
            .build()
            .await?;
            
        if session_proxy.active().await?
            && matches!(session_proxy.session_type().await?.as_str(), "x11" | "wayland")
        {
            let (uid, _user_path) = session_proxy.user().await?;
            debug!(uid, "Found active graphical session for user.");
            active_users.insert(TargetUser { uid, username });
        }
    }
    Ok(active_users)
}

async fn send_notification_to_user(
    user: &TargetUser,
    summary: &str,
    body: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    // FIX: Use `parse()` to convert a string to an Address
    let dbus_address: Address = format!("unix:path=/run/user/{}/bus", user.uid).parse()?;
    
    let user_session_bus = zbus::connection::Builder::address(dbus_address)?
        .build()
        .await?;

    let notifications_proxy = NotificationsProxy::new(&user_session_bus).await?;
    let notification_id = notifications_proxy
        .notify(
            "System Notifier",
            0,
            "dialog-information-symbolic",
            summary,
            body,
            &[],
            &HashMap::new(),
            -1,
        )
        .await?;
    Ok(notification_id)
}

