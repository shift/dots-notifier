//! D-Bus interface definitions and proxy traits

use std::collections::HashMap;
use zbus::{zvariant::{OwnedObjectPath, Value}, Result as ZbusResult};

/// D-Bus interface name for the notifier service
pub const DBUS_INTERFACE_NAME: &str = "me.section.Notifier";

/// D-Bus path for the notifier service
pub const DBUS_PATH: &str = "/me/section/Notifier";

/// Proxy trait for systemd login manager
#[zbus::proxy(
    interface = "org.freedesktop.login1.Manager",
    default_service = "org.freedesktop.login1",
    default_path = "/org/freedesktop/login1"
)]
pub trait LoginManager {
    #[zbus(name = "ListSessions")]
    fn list_sessions(&self) -> ZbusResult<Vec<SessionInfo>>;
}

/// Type alias for session information returned by LoginManager
pub type SessionInfo = (String, u32, String, String, OwnedObjectPath);

/// Proxy trait for systemd login session
#[zbus::proxy(
    interface = "org.freedesktop.login1.Session",
    default_service = "org.freedesktop.login1"
)]
pub trait Session {
    #[zbus(property)]
    fn active(&self) -> ZbusResult<bool>;

    #[zbus(property, name = "Type")]
    fn session_type(&self) -> ZbusResult<String>;

    #[zbus(property)]
    fn user(&self) -> ZbusResult<(u32, OwnedObjectPath)>;
}

/// Proxy trait for freedesktop notifications
#[zbus::proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
pub trait Notifications {
    /// Send a notification
    #[allow(clippy::too_many_arguments)]
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
    ) -> ZbusResult<u32>;
}

/// Proxy trait for the notifier client
#[zbus::proxy(
    interface = "me.section.Notifier",
    default_service = "me.section.Notifier",
    default_path = "/me/section/Notifier"
)]
pub trait Notifier {
    async fn send_to_all(&self, title: &str, body: &str) -> ZbusResult<()>;
}

/// Helper function to determine if a session type is graphical
pub fn is_graphical_session(session_type: &str) -> bool {
    matches!(session_type, "x11" | "wayland")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(DBUS_INTERFACE_NAME, "me.section.Notifier");
        assert_eq!(DBUS_PATH, "/me/section/Notifier");
    }

    #[test]
    fn test_is_graphical_session() {
        // Test valid graphical session types
        assert!(is_graphical_session("x11"));
        assert!(is_graphical_session("wayland"));

        // Test invalid session types
        assert!(!is_graphical_session("tty"));
        assert!(!is_graphical_session("console"));
        assert!(!is_graphical_session(""));
        assert!(!is_graphical_session("unknown"));
        assert!(!is_graphical_session("mir"));
    }

    #[test]
    fn test_is_graphical_session_case_sensitivity() {
        // Test case sensitivity
        assert!(!is_graphical_session("X11"));
        assert!(!is_graphical_session("WAYLAND"));
        assert!(!is_graphical_session("Wayland"));
    }

    #[test]
    fn test_session_info_type() {
        // Test that SessionInfo is the correct type
        let session_info: SessionInfo = (
            "session-1".to_string(),
            1000,
            "testuser".to_string(),
            "seat0".to_string(),
            "/org/freedesktop/login1/session/_31".try_into().unwrap(),
        );

        assert_eq!(session_info.0, "session-1");
        assert_eq!(session_info.1, 1000);
        assert_eq!(session_info.2, "testuser");
        assert_eq!(session_info.3, "seat0");
    }

    #[test]
    fn test_graphical_session_edge_cases() {
        // Test edge cases
        assert!(!is_graphical_session("x11_extra"));
        assert!(!is_graphical_session("wayland_extra"));
        assert!(!is_graphical_session("_x11"));
        assert!(!is_graphical_session("_wayland"));
        assert!(!is_graphical_session("x11\n"));
        assert!(!is_graphical_session("wayland\t"));
        assert!(!is_graphical_session(" x11"));
        assert!(!is_graphical_session(" wayland"));
    }
}