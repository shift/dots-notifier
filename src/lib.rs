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
use std::process::Command;

use crate::session::get_active_graphical_users;
use crate::types::TargetUser;

/// Validate that a string is safe to use as a command argument.
/// This prevents command injection by rejecting strings with shell metacharacters.
fn validate_safe_argument(arg: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check for shell metacharacters that could be used for injection
    const DANGEROUS_CHARS: &[char] = &['&', '|', ';', '`', '$', '(', ')', '{', '}', '[', ']', '<', '>', '\\', '\n', '\r', '\t'];
    
    if arg.contains(DANGEROUS_CHARS) {
        return Err(format!("Argument contains dangerous characters: {}", arg).into());
    }
    
    // Also reject arguments that start with dashes to prevent flag injection
    if arg.starts_with('-') {
        return Err(format!("Argument cannot start with dash: {}", arg).into());
    }
    
    Ok(())
}

/// Send a notification to a user via the helper process
async fn send_notification_via_helper(
    user: &TargetUser,
    title: &str,
    body: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Validate input arguments to prevent command injection
    validate_safe_argument(title)?;
    validate_safe_argument(body)?;
    validate_safe_argument(user.username())?;
    
    // Try systemd-run first (more reliable for user services)
    let mut cmd = Command::new("systemd-run");
    cmd.args([
        "--user",
        "--uid", &user.uid().to_string(),
        "--gid", &user.uid().to_string(), // Assume same gid as uid for simplicity
        "--setenv=USER", user.username(),
        "--setenv=USERNAME", user.username(),
        "dots-notifier-helper",
        title,
        body,
    ]);

    let output = match cmd.output() {
        Ok(output) => output,
        Err(e) => {
            // Fall back to sudo if systemd-run is not available
            warn!("systemd-run failed ({}), falling back to sudo", e);
            let mut sudo_cmd = Command::new("sudo");
            sudo_cmd.args([
                "-u", user.username(),
                "dots-notifier-helper",
                title,
                body,
            ]);
            sudo_cmd.output()?
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Helper process failed: {}", stderr).into());
    }

    Ok(())
}

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
                if let Err(e) = send_notification_via_helper(&user, &title_clone, &body_clone).await {
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

    #[test]
    fn test_validate_safe_argument() {
        // Valid arguments should pass
        assert!(validate_safe_argument("hello").is_ok());
        assert!(validate_safe_argument("hello world").is_ok());
        assert!(validate_safe_argument("123").is_ok());
        
        // Arguments with dangerous characters should fail
        assert!(validate_safe_argument("hello & world").is_err());
        assert!(validate_safe_argument("hello | world").is_err());
        assert!(validate_safe_argument("hello; world").is_err());
        assert!(validate_safe_argument("hello`world").is_err());
        assert!(validate_safe_argument("hello$world").is_err());
        assert!(validate_safe_argument("hello(world)").is_err());
        assert!(validate_safe_argument("hello{world}").is_err());
        assert!(validate_safe_argument("hello[world]").is_err());
        assert!(validate_safe_argument("hello<world>").is_err());
        assert!(validate_safe_argument("hello\\world").is_err());
        assert!(validate_safe_argument("hello\nworld").is_err());
        assert!(validate_safe_argument("hello\rworld").is_err());
        assert!(validate_safe_argument("hello\tworld").is_err());
        
        // Arguments starting with dashes should fail
        assert!(validate_safe_argument("-flag").is_err());
        assert!(validate_safe_argument("--option").is_err());
    }

    #[tokio::test]
    async fn test_send_notification_via_helper_with_invalid_user() {
        // Test that the helper function handles invalid users gracefully
        let invalid_user = TargetUser::new(99999, "nonexistent_user_12345".to_string());
        let result = send_notification_via_helper(&invalid_user, "Test", "Test").await;
        
        // Should fail because the user doesn't exist
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_send_notification_via_helper_with_invalid_args() {
        // Test that dangerous arguments are rejected
        let user = TargetUser::new(1000, "testuser".to_string());
        
        // Test command injection attempts
        assert!(send_notification_via_helper(&user, "test & rm -rf /", "body").await.is_err());
        assert!(send_notification_via_helper(&user, "title", "body | cat /etc/passwd").await.is_err());
        assert!(send_notification_via_helper(&user, "title", "body; echo dangerous").await.is_err());
        assert!(send_notification_via_helper(&user, "--help", "body").await.is_err());
    }
}