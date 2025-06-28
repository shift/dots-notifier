//! Core types used throughout the application

use std::fmt;

/// Represents a target user for notifications
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TargetUser {
    pub uid: u32,
    pub username: String,
}

impl TargetUser {
    /// Create a new TargetUser
    pub fn new(uid: u32, username: String) -> Self {
        Self { uid, username }
    }

    /// Get the user ID
    pub fn uid(&self) -> u32 {
        self.uid
    }

    /// Get the username
    pub fn username(&self) -> &str {
        &self.username
    }
}

impl fmt::Display for TargetUser {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.username, self.uid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_target_user_creation() {
        let user = TargetUser::new(1000, "testuser".to_string());
        assert_eq!(user.uid(), 1000);
        assert_eq!(user.username(), "testuser");
    }

    #[test]
    fn test_target_user_equality() {
        let user1 = TargetUser::new(1000, "testuser".to_string());
        let user2 = TargetUser::new(1000, "testuser".to_string());
        let user3 = TargetUser::new(1001, "testuser".to_string());
        let user4 = TargetUser::new(1000, "otheruser".to_string());

        assert_eq!(user1, user2);
        assert_ne!(user1, user3);
        assert_ne!(user1, user4);
    }

    #[test]
    fn test_target_user_hash() {
        let user1 = TargetUser::new(1000, "testuser".to_string());
        let user2 = TargetUser::new(1000, "testuser".to_string());
        let user3 = TargetUser::new(1001, "testuser".to_string());

        let mut set = HashSet::new();
        set.insert(user1.clone());
        set.insert(user2.clone());
        set.insert(user3.clone());

        // Should only have 2 unique users since user1 and user2 are equal
        assert_eq!(set.len(), 2);
        assert!(set.contains(&user1));
        assert!(set.contains(&user3));
    }

    #[test]
    fn test_target_user_clone() {
        let user = TargetUser::new(1000, "testuser".to_string());
        let cloned = user.clone();
        assert_eq!(user, cloned);
    }

    #[test]
    fn test_target_user_debug() {
        let user = TargetUser::new(1000, "testuser".to_string());
        let debug_str = format!("{:?}", user);
        assert!(debug_str.contains("1000"));
        assert!(debug_str.contains("testuser"));
    }

    #[test]
    fn test_target_user_display() {
        let user = TargetUser::new(1000, "testuser".to_string());
        let display_str = format!("{}", user);
        assert_eq!(display_str, "testuser(1000)");
    }

    #[test]
    fn test_target_user_edge_cases() {
        // Test with empty username
        let user = TargetUser::new(0, "".to_string());
        assert_eq!(user.uid(), 0);
        assert_eq!(user.username(), "");
        assert_eq!(format!("{}", user), "(0)");

        // Test with special characters in username
        let user = TargetUser::new(1000, "test-user_123".to_string());
        assert_eq!(user.username(), "test-user_123");
        assert_eq!(format!("{}", user), "test-user_123(1000)");

        // Test with very large UID
        let user = TargetUser::new(u32::MAX, "maxuser".to_string());
        assert_eq!(user.uid(), u32::MAX);
        assert_eq!(format!("{}", user), format!("maxuser({})", u32::MAX));
    }

    #[test]
    fn test_target_user_unicode() {
        let user = TargetUser::new(1000, "用户".to_string());
        assert_eq!(user.username(), "用户");
        assert_eq!(format!("{}", user), "用户(1000)");
    }
}