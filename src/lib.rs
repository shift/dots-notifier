//! # dots-notifier
//!
//! A client/server tool to send notifications to all active graphical users on a system.
//!
//! This library provides the core functionality for the dots-notifier application,
//! including D-Bus communication, user session detection, and notification dispatch.

pub mod cli;
pub mod dbus;
pub mod notification;
pub mod session;
pub mod types;

#[cfg(test)]
mod proptests;

use futures::future::join_all;
use tracing::{error, info, warn};
use zbus::interface;

use crate::session::get_active_graphical_users;
use crate::notification::send_notification_to_user;

/// The main NotifierService implementation for D-Bus interface.
#[derive(Debug, Default)]
pub struct NotifierService;

#[interface(name = "me.section.Notifier")]
impl NotifierService {
    /// Send notifications to all active graphical users.
    /// 
    /// # Arguments
    /// * `title` - The notification title
    /// * `body` - The notification body text
    /// 
    /// # Returns
    /// A D-Bus result indicating success or failure
    pub async fn send_to_all(&self, title: String, body: String) -> zbus::fdo::Result<()> {
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
            async move {
                let user_span = tracing::info_span!("user_notification", uid = user.uid, username = %user.username);
                let _enter = user_span.enter();
                if let Err(e) = send_notification_to_user(&user, &title_clone, &body_clone).await {
                    error!("Failed to send notification: {}", e);
                } else {
                    info!("Notification sent successfully.");
                }
            }
        });

        join_all(notification_tasks).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifier_service_creation() {
        let service = NotifierService;
        let debug_str = format!("{:?}", service);
        assert!(!debug_str.is_empty());
    }
}