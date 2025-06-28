//! Integration tests for dots-notifier
//!
//! These tests focus on testing the interaction between components
//! and simulating real-world scenarios.

use std::collections::HashSet;

use dots_notifier::{
    types::TargetUser,
    cli::{Cli, Commands},
    notification::{validate_notification_content, NotificationBuilder},
    session::filter_graphical_sessions,
    dbus::is_graphical_session,
    NotifierService,
};

/// Test the full CLI parsing flow
#[test]
fn test_cli_integration() {
    // Test server command
    let cli = Cli::try_parse_from(["dots-notifier", "server"]).unwrap();
    match cli.command {
        Commands::Server => {}, // Expected
        _ => panic!("Expected Server command"),
    }

    // Test send command
    let cli = Cli::try_parse_from(["dots-notifier", "send", "Hello", "World"]).unwrap();
    match cli.command {
        Commands::Send { title, body } => {
            assert_eq!(title, "Hello");
            assert_eq!(body, "World");
        }
        _ => panic!("Expected Send command"),
    }
}

/// Test user collection and filtering
#[test]
fn test_user_session_filtering() {
    let sessions = [
        ("x11", true, "session-1"),
        ("wayland", true, "session-2"),
        ("tty", true, "session-3"),
        ("x11", false, "session-4"),
        ("console", true, "session-5"),
    ];

    let graphical_sessions = filter_graphical_sessions(
        sessions.iter().map(|(t, a, s)| (*t, *a, *s))
    );

    assert_eq!(graphical_sessions.len(), 2);
    assert!(graphical_sessions.contains(&"session-1"));
    assert!(graphical_sessions.contains(&"session-2"));
}

/// Test TargetUser collection behavior
#[test]
fn test_target_user_collection() {
    let mut users = HashSet::new();
    
    // Add some users
    users.insert(TargetUser::new(1000, "alice".to_string()));
    users.insert(TargetUser::new(1001, "bob".to_string()));
    users.insert(TargetUser::new(1000, "alice".to_string())); // Duplicate
    
    assert_eq!(users.len(), 2); // Should only have 2 unique users
    assert!(users.contains(&TargetUser::new(1000, "alice".to_string())));
    assert!(users.contains(&TargetUser::new(1001, "bob".to_string())));
}

/// Test notification validation edge cases
#[test]
fn test_notification_validation_boundaries() {
    // Test exactly at the boundary
    let max_summary = "a".repeat(1000);
    let max_body = "b".repeat(5000);
    assert!(validate_notification_content(&max_summary, &max_body).is_ok());

    // Test just over the boundary
    let over_summary = "a".repeat(1001);
    let over_body = "b".repeat(5001);
    assert!(validate_notification_content(&over_summary, "").is_err());
    assert!(validate_notification_content("a", &over_body).is_err());

    // Test empty cases
    assert!(validate_notification_content("", "body").is_err());
    assert!(validate_notification_content("summary", "").is_ok());
}

/// Test NotificationBuilder fluent interface
#[test]
fn test_notification_builder_integration() {
    let builder = NotificationBuilder::new("Test", "Content")
        .app_name("Test App")
        .icon("test-icon")
        .timeout(5000)
        .action("action1", "Action 1")
        .hint("urgency", "critical");

    // We can't test the actual sending without D-Bus, but we can test the builder
    assert!(format!("{:?}", builder).contains("Test"));
}

/// Test session type detection
#[test]
fn test_session_type_detection_comprehensive() {
    // Valid graphical sessions
    assert!(is_graphical_session("x11"));
    assert!(is_graphical_session("wayland"));

    // Invalid sessions
    let invalid_sessions = [
        "tty", "console", "mir", "unknown", "",
        "x11_modified", "wayland_modified", 
        " x11", "x11 ", "X11", "WAYLAND",
        "x11\n", "wayland\t"
    ];

    for session in &invalid_sessions {
        assert!(!is_graphical_session(session), "Session '{}' should not be graphical", session);
    }
}

/// Test NotifierService creation and basic interface
#[test]
fn test_notifier_service_interface() {
    let service = NotifierService;
    
    // Test that it implements Debug
    let debug_str = format!("{:?}", service);
    assert!(!debug_str.is_empty());
}

/// Stress test for user collections with many duplicates
#[test]
fn test_large_user_collection() {
    let mut users = HashSet::new();
    
    // Add many duplicate users
    for i in 0..1000 {
        users.insert(TargetUser::new(i % 10, format!("user{}", i % 10)));
    }
    
    // Should only have 10 unique users
    assert_eq!(users.len(), 10);
}

/// Test with unicode and special characters
#[test]
fn test_unicode_handling() {
    let user = TargetUser::new(1000, "用户名".to_string());
    assert_eq!(user.username(), "用户名");
    assert_eq!(format!("{}", user), "用户名(1000)");

    // Test notification content with unicode
    assert!(validate_notification_content("🚀 Title", "Unicode content: 测试").is_ok());
}

/// Test error scenarios
#[test]
fn test_error_scenarios() {
    // Test various invalid inputs that should fail gracefully
    
    // Invalid notification content
    assert!(validate_notification_content("", "body").is_err());
    let too_long = "x".repeat(10000);
    assert!(validate_notification_content(&too_long, "").is_err());
    assert!(validate_notification_content("title", &too_long).is_err());
}

/// Performance test for session filtering
#[test]
fn test_session_filtering_performance() {
    let sessions: Vec<_> = (0..10000)
        .map(|i| {
            let session_type = match i % 4 {
                0 => "x11",
                1 => "wayland", 
                2 => "tty",
                _ => "console",
            };
            let is_active = i % 2 == 0;
            let session_id = format!("session-{}", i);
            (session_type, is_active, session_id)
        })
        .collect();

    let start = std::time::Instant::now();
    let filtered = filter_graphical_sessions(
        sessions.iter().map(|(t, a, s)| (*t, *a, s.as_str()))
    );
    let duration = start.elapsed();

    // Should complete quickly (under 10ms for 10k items)
    assert!(duration.as_millis() < 10);
    
    // Should have filtered correctly (half active, half of those are graphical)
    assert_eq!(filtered.len(), 2500); // 10000 / 2 (active) / 2 (graphical types)
}

/// Test concurrent safety with user collections
#[tokio::test]
async fn test_concurrent_user_operations() {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    let users = Arc::new(Mutex::new(HashSet::new()));
    let mut handles = vec![];
    
    // Spawn multiple tasks that add users concurrently
    for i in 0..100 {
        let users_clone = Arc::clone(&users);
        let handle = tokio::spawn(async move {
            let user = TargetUser::new(i, format!("user{}", i));
            users_clone.lock().await.insert(user);
        });
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
    
    // Should have 100 unique users
    let final_users = users.lock().await;
    assert_eq!(final_users.len(), 100);
}