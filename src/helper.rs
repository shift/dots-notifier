//! Helper binary for sending notifications from user context
//! 
//! This binary runs as the user and has access to the user's session bus,
//! allowing it to send notifications without "Broken pipe" errors.

use std::error::Error;
use std::env;
use tracing::{info, error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

use dots_notifier::{
    notification::send_notification_to_user,
    types::TargetUser,
};

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

    // Get current user information
    let uid = unsafe { libc::getuid() };
    let username = match env::var("USER").or_else(|_| env::var("USERNAME")) {
        Ok(name) => name,
        Err(_) => {
            error!("Could not determine username from environment");
            std::process::exit(1);
        }
    };

    let current_user = TargetUser::new(uid, username);
    
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
    #[test]
    fn test_helper_binary_exists() {
        // This test just ensures the helper binary can be compiled
        // Real testing requires a D-Bus session which is not available in unit tests
    }
}