//! Notification sending functionality

use std::collections::HashMap;
use zbus::Address;

use crate::types::TargetUser;
use crate::dbus::NotificationsProxy;

/// Send a notification to a specific user's session bus
pub async fn send_notification_to_user(
    user: &TargetUser,
    summary: &str,
    body: &str,
) -> Result<u32, Box<dyn std::error::Error>> {
    let dbus_address: Address = format!("unix:path=/run/user/{}/bus", user.uid()).parse()?;
    
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

/// Create a notification with custom parameters
#[derive(Debug)]
pub struct NotificationBuilder {
    app_name: String,
    replaces_id: u32,
    app_icon: String,
    summary: String,
    body: String,
    actions: Vec<String>,
    hints: HashMap<String, String>,
    expire_timeout: i32,
}

impl NotificationBuilder {
    /// Create a new notification builder with default values
    pub fn new(summary: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            app_name: "System Notifier".to_string(),
            replaces_id: 0,
            app_icon: "dialog-information-symbolic".to_string(),
            summary: summary.into(),
            body: body.into(),
            actions: Vec::new(),
            hints: HashMap::new(),
            expire_timeout: -1,
        }
    }

    /// Set the application name
    pub fn app_name(mut self, app_name: impl Into<String>) -> Self {
        self.app_name = app_name.into();
        self
    }

    /// Set the icon name
    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.app_icon = icon.into();
        self
    }

    /// Set the expiration timeout
    pub fn timeout(mut self, timeout: i32) -> Self {
        self.expire_timeout = timeout;
        self
    }

    /// Add an action
    pub fn action(mut self, key: impl Into<String>, label: impl Into<String>) -> Self {
        self.actions.push(key.into());
        self.actions.push(label.into());
        self
    }

    /// Add a hint
    pub fn hint(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.hints.insert(key.into(), value.into());
        self
    }

    /// Send the notification to a user
    pub async fn send_to_user(self, user: &TargetUser) -> Result<u32, Box<dyn std::error::Error>> {
        let dbus_address: Address = format!("unix:path=/run/user/{}/bus", user.uid()).parse()?;
        
        let user_session_bus = zbus::connection::Builder::address(dbus_address)?
            .build()
            .await?;

        let notifications_proxy = NotificationsProxy::new(&user_session_bus).await?;
        
        // Convert actions to slice of string refs
        let action_refs: Vec<&str> = self.actions.iter().map(|s| s.as_str()).collect();
        
        // Convert hints to the required format
        let hint_refs: HashMap<&str, zbus::zvariant::Value<'_>> = self.hints
            .iter()
            .map(|(k, v)| (k.as_str(), zbus::zvariant::Value::from(v.as_str())))
            .collect();

        let notification_id = notifications_proxy
            .notify(
                &self.app_name,
                self.replaces_id,
                &self.app_icon,
                &self.summary,
                &self.body,
                &action_refs,
                &hint_refs,
                self.expire_timeout,
            )
            .await?;
        Ok(notification_id)
    }
}

/// Validate notification content
pub fn validate_notification_content(summary: &str, body: &str) -> Result<(), Box<dyn std::error::Error>> {
    if summary.is_empty() {
        return Err("Notification summary cannot be empty".into());
    }
    
    if summary.len() > 1000 {
        return Err("Notification summary too long (max 1000 characters)".into());
    }
    
    if body.len() > 5000 {
        return Err("Notification body too long (max 5000 characters)".into());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notification_builder_defaults() {
        let builder = NotificationBuilder::new("Test Summary", "Test Body");
        assert_eq!(builder.summary, "Test Summary");
        assert_eq!(builder.body, "Test Body");
        assert_eq!(builder.app_name, "System Notifier");
        assert_eq!(builder.app_icon, "dialog-information-symbolic");
        assert_eq!(builder.replaces_id, 0);
        assert_eq!(builder.expire_timeout, -1);
        assert!(builder.actions.is_empty());
        assert!(builder.hints.is_empty());
    }

    #[test]
    fn test_notification_builder_customization() {
        let builder = NotificationBuilder::new("Summary", "Body")
            .app_name("Custom App")
            .icon("custom-icon")
            .timeout(5000)
            .action("action1", "Action 1")
            .action("action2", "Action 2")
            .hint("urgency", "critical")
            .hint("category", "device");

        assert_eq!(builder.app_name, "Custom App");
        assert_eq!(builder.app_icon, "custom-icon");
        assert_eq!(builder.expire_timeout, 5000);
        assert_eq!(builder.actions, vec!["action1", "Action 1", "action2", "Action 2"]);
        assert_eq!(builder.hints.get("urgency"), Some(&"critical".to_string()));
        assert_eq!(builder.hints.get("category"), Some(&"device".to_string()));
    }

    #[test]
    fn test_validate_notification_content_valid() {
        assert!(validate_notification_content("Valid summary", "Valid body").is_ok());
        assert!(validate_notification_content("S", "").is_ok());
        
        let long_summary = "a".repeat(1000);
        let long_body = "b".repeat(5000);
        assert!(validate_notification_content(&long_summary, &long_body).is_ok());
    }

    #[test]
    fn test_validate_notification_content_invalid() {
        // Empty summary
        assert!(validate_notification_content("", "body").is_err());
        
        // Summary too long
        let too_long_summary = "a".repeat(1001);
        assert!(validate_notification_content(&too_long_summary, "body").is_err());
        
        // Body too long
        let too_long_body = "b".repeat(5001);
        assert!(validate_notification_content("summary", &too_long_body).is_err());
    }

    #[test]
    fn test_validate_notification_content_edge_cases() {
        // Exactly at limits
        let max_summary = "a".repeat(1000);
        let max_body = "b".repeat(5000);
        assert!(validate_notification_content(&max_summary, &max_body).is_ok());
        
        // Unicode characters
        assert!(validate_notification_content("测试", "测试内容").is_ok());
        
        // Special characters
        assert!(validate_notification_content("Title with \"quotes\"", "Body with\nnewlines\ttabs").is_ok());
    }

    // Note: send_notification_to_user() and NotificationBuilder::send_to_user() 
    // require actual D-Bus connection and are tested in integration tests
}