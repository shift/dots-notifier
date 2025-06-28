//! Command-line argument parsing module

use clap::{Parser, Subcommand};

/// Command-line interface definition
#[derive(Parser, Debug, Clone, PartialEq)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands for the application
#[derive(Subcommand, Debug, Clone, PartialEq)]
pub enum Commands {
    /// Run in server mode, listening for D-Bus requests. (For systemd/D-Bus activation)
    Server,
    /// Send a notification to all users.
    Send {
        /// The title of the notification.
        title: String,
        /// The body message of the notification.
        body: String,
    },
}

impl Cli {
    /// Parse command line arguments
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }

    /// Parse command line arguments from an iterator
    pub fn try_parse_from<I, T>(itr: I) -> Result<Self, clap::Error> 
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        <Self as Parser>::try_parse_from(itr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_server_command() {
        let cli = Cli::try_parse_from(["test", "server"]).unwrap();
        assert_eq!(cli.command, Commands::Server);
    }

    #[test]
    fn test_cli_send_command() {
        let cli = Cli::try_parse_from(["test", "send", "Test Title", "Test Body"]).unwrap();
        assert_eq!(cli.command, Commands::Send {
            title: "Test Title".to_string(),
            body: "Test Body".to_string(),
        });
    }

    #[test]
    fn test_cli_send_command_with_spaces() {
        let cli = Cli::try_parse_from(["test", "send", "Title with spaces", "Body with spaces"]).unwrap();
        assert_eq!(cli.command, Commands::Send {
            title: "Title with spaces".to_string(),
            body: "Body with spaces".to_string(),
        });
    }

    #[test]
    fn test_cli_invalid_command() {
        let result = Cli::try_parse_from(["test", "invalid"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_send_missing_args() {
        let result = Cli::try_parse_from(["test", "send", "title"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_no_args() {
        let result = Cli::try_parse_from(["test"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_cli_debug_format() {
        let cli = Cli::try_parse_from(["test", "server"]).unwrap();
        let debug_str = format!("{:?}", cli);
        assert!(debug_str.contains("Server"));
    }

    #[test]
    fn test_cli_clone() {
        let cli = Cli::try_parse_from(["test", "server"]).unwrap();
        let cloned = cli.clone();
        assert_eq!(cli, cloned);
    }

    #[test]
    fn test_commands_debug_format() {
        let cmd = Commands::Server;
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Server"));

        let cmd = Commands::Send {
            title: "Test".to_string(),
            body: "Body".to_string(),
        };
        let debug_str = format!("{:?}", cmd);
        assert!(debug_str.contains("Send"));
        assert!(debug_str.contains("Test"));
        assert!(debug_str.contains("Body"));
    }
}