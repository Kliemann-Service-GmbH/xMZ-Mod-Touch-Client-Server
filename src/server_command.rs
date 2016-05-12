#![feature(slice_patterns)]
use std::str::FromStr;
/// Store the server command
///
/// ** Parameters: **
/// - Command: the command (led, relais, ...)
/// - Action: the action for the command (set, get, toggle, ...)
/// - Value: the value for the action (1, 4, 10 ) // TODO: implement ranges like 4..11
#[derive(PartialEq)]
pub struct ServerCommand {
    command: String,
    action: String,
    value: String,
}

impl ServerCommand {
    pub fn new(c: String, a: String, v: String) -> ServerCommand {
        ServerCommand {
            command: c, action: a, value: v
        }
    }
}

impl FromStr for ServerCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = s.split_whitespace().collect();
        match &split[..] {
            [command, action, value] => Ok(ServerCommand::new(command.to_string(), action.to_string(), value.to_string())),
            _ => Err(format!("Invalid server command: {}", s)),
        }
    }
}

#[cfg(test)]
mod servercommand_tests {
    use super::*;

    #[test]
    fn can_parse_full_servercommand() {
        let cmd = ServerCommand::new("led".to_string(), "set".to_string(), "1".to_string());
        assert!(cmd == "led set 1".parse().unwrap());
    }
}
