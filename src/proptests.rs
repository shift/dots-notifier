//! Property-based tests using proptest

use proptest::prelude::*;
use std::collections::HashSet;

use crate::types::TargetUser;
use crate::notification::validate_notification_content;
use crate::dbus::is_graphical_session;

proptest! {
    #[test]
    fn test_target_user_uid_never_changes(uid: u32, username: String) {
        let user = TargetUser::new(uid, username);
        prop_assert_eq!(user.uid(), uid);
    }

    #[test]
    fn test_target_user_username_never_changes(uid: u32, username: String) {
        let user = TargetUser::new(uid, username.clone());
        prop_assert_eq!(user.username(), &username);
    }

    #[test]
    fn test_target_user_equality_is_consistent(uid: u32, username: String) {
        let user1 = TargetUser::new(uid, username.clone());
        let user2 = TargetUser::new(uid, username);
        prop_assert_eq!(user1, user2);
    }

    #[test]
    fn test_target_user_hash_consistency(uid: u32, username: String) {
        let user1 = TargetUser::new(uid, username.clone());
        let user2 = TargetUser::new(uid, username);
        
        let mut set = HashSet::new();
        set.insert(user1);
        set.insert(user2);
        
        // Should only have 1 unique user since they're equal
        prop_assert_eq!(set.len(), 1);
    }

    #[test]
    fn test_target_user_display_format(uid: u32, username: String) {
        let user = TargetUser::new(uid, username.clone());
        let display = format!("{}", user);
        prop_assert!(display.contains(&username));
        prop_assert!(display.contains(&uid.to_string()));
    }

    #[test]
    fn test_valid_notification_content(
        summary in "[a-zA-Z0-9 ]{1,999}",
        body in "[a-zA-Z0-9 \\n\\t]*"
    ) {
        prop_assume!(body.len() <= 5000);
        prop_assert!(validate_notification_content(&summary, &body).is_ok());
    }

    #[test]
    fn test_invalid_long_summary(
        summary in "[a-zA-Z]{1001,2000}",
        body in "[a-zA-Z0-9 ]*"
    ) {
        prop_assume!(body.len() <= 5000);
        prop_assert!(validate_notification_content(&summary, &body).is_err());
    }

    #[test]
    fn test_invalid_long_body(
        summary in "[a-zA-Z0-9 ]{1,999}",
        body in "[a-zA-Z]{5001,6000}"
    ) {
        prop_assert!(validate_notification_content(&summary, &body).is_err());
    }

    #[test]
    fn test_graphical_session_recognition(session_type: String) {
        let result = is_graphical_session(&session_type);
        if session_type == "x11" || session_type == "wayland" {
            prop_assert!(result);
        } else {
            prop_assert!(!result);
        }
    }

    #[test]
    fn test_target_user_clone_equality(uid: u32, username: String) {
        let user = TargetUser::new(uid, username);
        let cloned = user.clone();
        prop_assert_eq!(user, cloned);
    }
}