//! QuickCheck-based property tests

use quickcheck::TestResult;
use quickcheck_macros::quickcheck;
use std::collections::HashSet;

use dots_notifier::{
    types::TargetUser,
    notification::validate_notification_content,
    dbus::is_graphical_session,
};

#[quickcheck]
fn qc_target_user_uid_consistency(uid: u32, username: String) -> bool {
    let user = TargetUser::new(uid, username);
    user.uid() == uid
}

#[quickcheck]
fn qc_target_user_username_consistency(uid: u32, username: String) -> bool {
    let user = TargetUser::new(uid, username.clone());
    user.username() == username
}

#[quickcheck]
fn qc_target_user_equality_reflexive(uid: u32, username: String) -> bool {
    let user = TargetUser::new(uid, username);
    user == user
}

#[quickcheck]
fn qc_target_user_equality_symmetric(uid: u32, username: String) -> bool {
    let user1 = TargetUser::new(uid, username.clone());
    let user2 = TargetUser::new(uid, username);
    (user1 == user2) == (user2 == user1)
}

#[quickcheck]
fn qc_target_user_equality_transitive(uid: u32, username: String) -> bool {
    let user1 = TargetUser::new(uid, username.clone());
    let user2 = TargetUser::new(uid, username.clone());
    let user3 = TargetUser::new(uid, username);
    
    if user1 == user2 && user2 == user3 {
        user1 == user3
    } else {
        true // Property only applies when both conditions are true
    }
}

#[quickcheck]
fn qc_target_user_hash_equality_consistency(uid: u32, username: String) -> bool {
    let user1 = TargetUser::new(uid, username.clone());
    let user2 = TargetUser::new(uid, username);
    
    let mut set = HashSet::new();
    set.insert(user1);
    set.insert(user2);
    
    // If users are equal, hash set should only contain one
    set.len() == 1
}

#[quickcheck]
fn qc_target_user_display_contains_info(uid: u32, username: String) -> bool {
    let user = TargetUser::new(uid, username.clone());
    let display = format!("{}", user);
    display.contains(&username) && display.contains(&uid.to_string())
}

#[quickcheck]
fn qc_target_user_clone_equality(uid: u32, username: String) -> bool {
    let user = TargetUser::new(uid, username);
    let cloned = user.clone();
    user == cloned
}

#[quickcheck]
fn qc_notification_validation_empty_summary_fails(body: String) -> bool {
    validate_notification_content("", &body).is_err()
}

#[quickcheck]
fn qc_notification_validation_long_summary_fails(body: String) -> TestResult {
    if body.len() > 5000 {
        return TestResult::discard();
    }
    
    let long_summary = "a".repeat(1001);
    TestResult::from_bool(validate_notification_content(&long_summary, &body).is_err())
}

#[quickcheck]
fn qc_notification_validation_long_body_fails(summary: String) -> TestResult {
    if summary.is_empty() || summary.len() > 1000 {
        return TestResult::discard();
    }
    
    let long_body = "b".repeat(5001);
    TestResult::from_bool(validate_notification_content(&summary, &long_body).is_err())
}

#[quickcheck]
fn qc_notification_validation_valid_content_passes() -> bool {
    validate_notification_content("Valid summary", "Valid body").is_ok()
}

#[quickcheck]
fn qc_session_type_detection_consistency(session_type: String) -> bool {
    let result1 = is_graphical_session(&session_type);
    let result2 = is_graphical_session(&session_type);
    result1 == result2
}

#[quickcheck]
fn qc_session_type_only_x11_wayland_graphical(session_type: String) -> bool {
    let is_graphical = is_graphical_session(&session_type);
    if session_type == "x11" || session_type == "wayland" {
        is_graphical
    } else {
        !is_graphical
    }
}

// Additional manual quickcheck tests using the quickcheck! macro
mod manual_tests {
    use super::*;
    use quickcheck::quickcheck;

    quickcheck! {
        fn prop_target_user_different_uid_not_equal(uid1: u32, uid2: u32, username: String) -> TestResult {
            if uid1 == uid2 {
                return TestResult::discard();
            }
            
            let user1 = TargetUser::new(uid1, username.clone());
            let user2 = TargetUser::new(uid2, username);
            
            TestResult::from_bool(user1 != user2)
        }

        fn prop_target_user_different_username_not_equal(uid: u32, username1: String, username2: String) -> TestResult {
            if username1 == username2 {
                return TestResult::discard();
            }
            
            let user1 = TargetUser::new(uid, username1);
            let user2 = TargetUser::new(uid, username2);
            
            TestResult::from_bool(user1 != user2)
        }

        fn prop_notification_validation_reasonable_lengths_pass(summary: String, body: String) -> TestResult {
            if summary.is_empty() || summary.len() > 1000 || body.len() > 5000 {
                return TestResult::discard();
            }
            
            TestResult::from_bool(validate_notification_content(&summary, &body).is_ok())
        }

        fn prop_target_user_hash_set_behavior(users: Vec<(u32, String)>) -> bool {
            let mut set = HashSet::new();
            let mut unique_count = 0;
            let mut seen = HashSet::new();
            
            for (uid, username) in users {
                let user = TargetUser::new(uid, username.clone());
                set.insert(user);
                
                let key = (uid, username);
                if seen.insert(key) {
                    unique_count += 1;
                }
            }
            
            set.len() == unique_count
        }

        fn prop_session_filtering_preserves_order(sessions: Vec<String>) -> bool {
            let graphical_sessions: Vec<_> = sessions.iter()
                .enumerate()
                .filter(|(_, session)| is_graphical_session(session))
                .map(|(i, _)| i)
                .collect();
            
            // Check that the order is preserved (should be ascending)
            graphical_sessions.windows(2).all(|w| w[0] < w[1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_properties() {
        // Test some basic properties manually
        let user = TargetUser::new(1000, "test".to_string());
        assert_eq!(user.uid(), 1000);
        assert_eq!(user.username(), "test");
        assert_eq!(user, user.clone());
        
        // Test session type consistency
        assert!(is_graphical_session("x11"));
        assert!(is_graphical_session("wayland"));
        assert!(!is_graphical_session("tty"));
        
        // Test notification validation
        assert!(validate_notification_content("Valid", "Valid").is_ok());
        assert!(validate_notification_content("", "body").is_err());
    }
}