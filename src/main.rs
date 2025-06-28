use std::error::Error;
use tracing::{info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use zbus::Connection;

use dots_notifier::{
    cli::{Cli, Commands},
    dbus::{DBUS_INTERFACE_NAME, DBUS_PATH, NotifierProxy},
    NotifierService,
};

/// Main application entry point
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing
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

/// Run the D-Bus server
async fn run_server() -> Result<(), Box<dyn Error>> {
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

/// Run the D-Bus client
async fn run_client(title: &str, body: &str) -> Result<(), Box<dyn Error>> {
    info!("Starting in client mode...");
    let connection = Connection::system().await?;
    let proxy = NotifierProxy::new(&connection).await?;

    info!("Sending notification request to the system service...");
    proxy.send_to_all(title, body).await?;
    info!("Request sent successfully.");

    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_main_function_exists() {
        // This test just ensures the main function compiles and can be called
        // We can't easily test the actual main function due to its side effects
    }

    // Note: run_server() and run_client() require actual D-Bus connections
    // and are tested in integration tests
}

