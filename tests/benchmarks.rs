//! Benchmark tests for performance measurements

use std::collections::HashSet;
use std::time::Instant;

use dots_notifier::{
    types::TargetUser,
    session::filter_graphical_sessions,
    notification::validate_notification_content,
    dbus::is_graphical_session,
};

/// Benchmark user creation and insertion
#[test]
fn bench_user_creation() {
    let start = Instant::now();
    let mut users = HashSet::new();
    
    for i in 0..10000 {
        let user = TargetUser::new(i, format!("user{}", i));
        users.insert(user);
    }
    
    let duration = start.elapsed();
    println!("Created and inserted 10k users in {:?}", duration);
    
    // Should be able to create 10k users quickly
    assert!(duration.as_millis() < 100);
    assert_eq!(users.len(), 10000);
}

/// Benchmark session filtering
#[test]
fn bench_session_filtering() {
    let sessions: Vec<_> = (0..100000)
        .map(|i| {
            let session_type = match i % 6 {
                0 => "x11",
                1 => "wayland",
                2 => "tty",
                3 => "console",
                4 => "mir",
                _ => "unknown",
            };
            let is_active = i % 3 == 0;
            let session_id = format!("session-{}", i);
            (session_type, is_active, session_id)
        })
        .collect();

    let start = Instant::now();
    let filtered = filter_graphical_sessions(
        sessions.iter().map(|(t, a, s)| (*t, *a, s.as_str()))
    );
    let duration = start.elapsed();
    
    println!("Filtered 100k sessions in {:?}, found {} sessions", duration, filtered.len());
    
    // Should complete quickly (under 100ms for 100k items)
    assert!(duration.as_millis() < 100);
    
    // Verify correctness: We expect some filtered sessions
    // The exact number depends on the algorithm
    assert!(!filtered.is_empty());
}

/// Benchmark notification validation
#[test]
fn bench_notification_validation() {
    let summary = "Test summary".to_string();
    let body = "Test body content that is reasonably long to simulate real notifications".to_string();
    
    let start = Instant::now();
    for _ in 0..100000 {
        validate_notification_content(&summary, &body).unwrap();
    }
    let duration = start.elapsed();
    
    println!("Validated 100k notifications in {:?}", duration);
    
    // Should validate quickly in CI environment
    assert!(duration.as_millis() < 100);
}

/// Benchmark session type checking
#[test]
fn bench_session_type_checking() {
    let session_types = ["x11", "wayland", "tty", "console", "mir", "unknown"];
    
    let start = Instant::now();
    for _ in 0..1000000 {
        for session_type in &session_types {
            is_graphical_session(session_type);
        }
    }
    let duration = start.elapsed();
    
    println!("Checked 6M session types in {:?}", duration);
    
    // Should be fast for simple string matching in CI
    assert!(duration.as_millis() < 500);
}

/// Benchmark user equality and hashing
#[test]
fn bench_user_operations() {
    let user1 = TargetUser::new(1000, "testuser".to_string());
    let user2 = TargetUser::new(1000, "testuser".to_string());
    let user3 = TargetUser::new(1001, "otheruser".to_string());
    
    let start = Instant::now();
    for _ in 0..1000000 {
        let _ = user1 == user2;
        let _ = user1 == user3;
        let _ = format!("{}", user1);
    }
    let duration = start.elapsed();
    
    println!("Performed 3M user operations in {:?}", duration);
    
    // Should be very fast for basic operations
    assert!(duration.as_millis() < 500);
}

/// Memory usage test with large collections
#[test]
fn bench_memory_usage() {
    let start = Instant::now();
    
    // Create a large collection to test memory efficiency
    let mut users = HashSet::new();
    for i in 0..50000 {
        users.insert(TargetUser::new(i, format!("user{}", i)));
    }
    
    // Test lookups
    let lookup_start = Instant::now();
    for i in 0..1000 {
        let user = TargetUser::new(i, format!("user{}", i));
        assert!(users.contains(&user));
    }
    let lookup_duration = lookup_start.elapsed();
    
    let total_duration = start.elapsed();
    
    println!("Created 50k users in {:?}, 1k lookups in {:?}", 
             total_duration, lookup_duration);
    
    // Should handle large collections efficiently
    assert!(total_duration.as_millis() < 500);
    assert!(lookup_duration.as_millis() < 10);
}

/// Stress test for concurrent operations simulation
#[test]
fn bench_concurrent_simulation() {
    use std::sync::Arc;
    use std::thread;
    
    let users = Arc::new(HashSet::<TargetUser>::new());
    let start = Instant::now();
    
    // Simulate concurrent access patterns
    let handles: Vec<_> = (0..10).map(|thread_id| {
        let _users_clone = Arc::clone(&users);
        thread::spawn(move || {
            let mut local_users = HashSet::new();
            for i in 0..1000 {
                let user = TargetUser::new(
                    thread_id * 1000 + i, 
                    format!("user{}-{}", thread_id, i)
                );
                local_users.insert(user);
            }
            local_users.len()
        })
    }).collect();
    
    let mut total_users = 0;
    for handle in handles {
        total_users += handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    
    println!("Simulated concurrent creation of {}k users in {:?}", 
             total_users / 1000, duration);
    
    assert_eq!(total_users, 10000);
    assert!(duration.as_millis() < 200);
}