//! User session detection and management

use std::collections::HashSet;
use tracing::{debug, debug_span};
use zbus::Connection;

use crate::types::TargetUser;
use crate::dbus::{LoginManagerProxy, SessionProxy, is_graphical_session};

/// Get all active graphical user sessions
pub async fn get_active_graphical_users() -> Result<HashSet<TargetUser>, Box<dyn std::error::Error>> {
    let mut active_users = HashSet::new();
    let sys_bus = Connection::system().await?;
    let manager_proxy = LoginManagerProxy::new(&sys_bus).await?;
    let sessions = manager_proxy.list_sessions().await?;
    
    for (session_id, _uid, username, _seat, session_path) in sessions {
        let session_span = debug_span!("session_check", id = %session_id, user = %username);
        let _enter = session_span.enter();

        let session_proxy = SessionProxy::builder(&sys_bus)
            .path(session_path)?
            .build()
            .await?;
            
        if session_proxy.active().await?
            && is_graphical_session(&session_proxy.session_type().await?)
        {
            let (uid, _user_path) = session_proxy.user().await?;
            debug!(uid, "Found active graphical session for user.");
            active_users.insert(TargetUser::new(uid, username));
        }
    }
    Ok(active_users)
}

/// Filter active sessions to only include graphical ones
pub fn filter_graphical_sessions<'a>(sessions: impl Iterator<Item = (&'a str, bool, &'a str)>) -> Vec<&'a str> {
    sessions
        .filter(|(session_type, is_active, _)| *is_active && is_graphical_session(session_type))
        .map(|(_, _, session_id)| session_id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_graphical_sessions() {
        let sessions = [
            ("x11", true, "session-1"),
            ("wayland", true, "session-2"),
            ("tty", true, "session-3"),
            ("x11", false, "session-4"),
            ("wayland", false, "session-5"),
            ("console", true, "session-6"),
        ];

        let filtered = filter_graphical_sessions(sessions.iter().map(|(t, a, s)| (*t, *a, *s)));
        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains(&"session-1"));
        assert!(filtered.contains(&"session-2"));
        assert!(!filtered.contains(&"session-3"));
        assert!(!filtered.contains(&"session-4"));
        assert!(!filtered.contains(&"session-5"));
        assert!(!filtered.contains(&"session-6"));
    }

    #[test]
    fn test_filter_graphical_sessions_empty() {
        let sessions: Vec<(&str, bool, &str)> = vec![];
        let filtered = filter_graphical_sessions(sessions.iter().map(|(t, a, s)| (*t, *a, *s)));
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_filter_graphical_sessions_no_active() {
        let sessions = [
            ("x11", false, "session-1"),
            ("wayland", false, "session-2"),
        ];

        let filtered = filter_graphical_sessions(sessions.iter().map(|(t, a, s)| (*t, *a, *s)));
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_filter_graphical_sessions_no_graphical() {
        let sessions = [
            ("tty", true, "session-1"),
            ("console", true, "session-2"),
        ];

        let filtered = filter_graphical_sessions(sessions.iter().map(|(t, a, s)| (*t, *a, *s)));
        assert!(filtered.is_empty());
    }

    #[test]
    fn test_filter_graphical_sessions_mixed() {
        let sessions = [
            ("x11", true, "session-1"),
            ("tty", true, "session-2"),
            ("wayland", false, "session-3"),
            ("console", false, "session-4"),
            ("x11", false, "session-5"),
            ("wayland", true, "session-6"),
        ];

        let filtered = filter_graphical_sessions(sessions.iter().map(|(t, a, s)| (*t, *a, *s)));
        assert_eq!(filtered.len(), 2);
        assert!(filtered.contains(&"session-1"));
        assert!(filtered.contains(&"session-6"));
    }

    // Note: get_active_graphical_users() requires actual D-Bus connection and is tested in integration tests
}