//! Helper binary for sending notifications from user context
//! 
//! This binary runs as the user and has access to the user's session bus,
//! allowing it to send notifications without "Broken pipe" errors.

use std::error::Error;
use std::env;
use tracing::{info, error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use nix::unistd::getuid;

use dots_notifier::{
    notification::send_notification_to_user,
    types::TargetUser,
};

/// Validate that a notification argument is safe and reasonable
fn validate_notification_argument(arg: &str, max_length: usize) -> Result<(), Box<dyn Error>> {
    if arg.is_empty() {
        return Err("Argument cannot be empty".into());
    }
    
    if arg.len() > max_length {
        return Err(format!("Argument too long (max {} chars): {}", max_length, arg.len()).into());
    }
    
    // Check for control characters that could cause issues
    if arg.chars().any(|c| c.is_control() && c != '\n' && c != '\t') {
        return Err("Argument contains invalid control characters".into());
    }
    
    Ok(())
}

/// Validate and sanitize username from environment
fn get_validated_username() -> Result<String, Box<dyn Error>> {
    let username = env::var("USER").or_else(|_| env::var("USERNAME"))
        .map_err(|_| "Could not determine username from environment")?;
    
    // Basic username validation - should be alphanumeric with some allowed special chars
    if username.is_empty() || username.len() > 32 {
        return Err("Invalid username length".into());
    }
    
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == '.') {
        return Err("Username contains invalid characters".into());
    }
    
    Ok(username)
}

/// Main entry point for the helper binary
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing with a more minimal setup for the helper
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("error"));
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: {} <title> <body>", args[0]);
        std::process::exit(1);
    }

    let title = &args[1];
    let body = &args[2];

    // Validate notification arguments
    validate_notification_argument(title, 100)?; // Limit title to 100 chars
    validate_notification_argument(body, 1000)?; // Limit body to 1000 chars

    // Get current user information safely
    let uid = getuid();
    let username = get_validated_username()?;

    let current_user = TargetUser::new(uid.as_raw(), username);
    
    info!("Sending notification as user {}", current_user);
    
    match send_notification_to_user(&current_user, title, body).await {
        Ok(_) => {
            info!("Notification sent successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to send notification: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper_binary_exists() {
        // This test just ensures the helper binary can be compiled
        // Real testing requires a D-Bus session which is not available in unit tests
    }

    #[test]
    fn test_validate_notification_argument() {
        // Valid arguments should pass
        assert!(validate_notification_argument("Hello", 100).is_ok());
        assert!(validate_notification_argument("Hello world", 100).is_ok());
        assert!(validate_notification_argument("123", 100).is_ok());
        assert!(validate_notification_argument("Unicode: 你好", 100).is_ok());
        
        // Empty arguments should fail
        assert!(validate_notification_argument("", 100).is_err());
        
        // Arguments that are too long should fail
        assert!(validate_notification_argument("a".repeat(101).as_str(), 100).is_err());
        
        // Arguments with control characters should fail (except \n and \t)
        assert!(validate_notification_argument("hello\x00world", 100).is_err());
        assert!(validate_notification_argument("hello\x1fworld", 100).is_err());
        
        // Newlines and tabs should be allowed
        assert!(validate_notification_argument("hello\nworld", 100).is_ok());
        assert!(validate_notification_argument("hello\tworld", 100).is_ok());
    }

    #[test]
    fn test_get_validated_username() {
        // Note: This test can't fully test the function because it depends on environment variables
        // But we can test the validation logic by setting environment variables
        
        // Test with invalid username lengths and characters would require mocking env vars
        // which is complex in Rust without additional dependencies
        
        // For now, just ensure the function exists and compiles
        let _ = get_validated_username();
    }
}