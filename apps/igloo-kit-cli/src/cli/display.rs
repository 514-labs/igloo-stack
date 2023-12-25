use console::{pad_str, style};
use std::sync::{Arc, RwLock};

/// # Display Module
/// Standardizes the way we display messages to the user in the CLI
///
/// ## Module Usage
/// ```
/// use igloo::cli::display::{MessageType, Message, show_message};
/// use std::sync::{RwLock, Arc};
///
/// let term = Arc::new(RwLock::new(CommandTerminal::new());
/// show_message(term.clone(), MessageType::Info, Message {
///    action: "Loading Config".to_string(),
///   details: "Reading configuration from ~/.igloo-config.toml".to_string(),
/// });
/// ```
///
/// ## Command Terminal
/// The CommandTerminal struct is used to keep track of the number of lines that have been written to the
/// terminal and gives a handle on the terminal itself to run commands like clearing the terminal.
///
/// ### Usage
/// ```
/// let term = Arc::new(RwLock::new(CommandTerminal::new());
/// show_message(term.clone(), MessageType::Info, Message {
///   action: "Loading Config".to_string(),
///   details: "Reading configuration from ~/.igloo-config.toml".to_string(),
/// });
/// term.clear();
/// ```
///
///
/// ## Message Types
/// - Info: blue action text and white details text. Used for general information.
/// - Success: green action text and white details text. Used for successful actions.
/// - Warning: yellow action text and white details text. Used for warnings.
/// - Error: red action text and white details text. Used for errors.
/// - Typographic: large stylistic text. Used for a text displays.
/// - Banner: multi line text that's used to display a banner that should drive an action from the user
///
/// ## Message Struct
/// ```
/// Message {
///    action: "Loading Config".to_string(),
///    details: "Reading configuration from ~/.igloo-config.toml".to_string(),
/// }
/// ```
///
/// ## Suggested Improvements
/// - remove the need for users to use .to_string() on the action and details fields
/// - add a message type for a "waiting" message
/// - add a message type for a "loading" message with a progress bar
/// - remove the arc and rwlock from show_message and instead pass in a reference to the terminal
///

#[derive(Debug, Clone)]
pub struct CommandTerminal {
    pub term: console::Term,
    pub counter: usize,
}

impl CommandTerminal {
    pub fn new() -> CommandTerminal {
        CommandTerminal {
            term: console::Term::stdout(),
            counter: 0,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MessageType {
    Info,
    Success,
    Error,
    Banner,
}

pub fn styled_banner() -> String {
    format!(
        r#"

---------------------------------------------------------------------------------------
{} 
We're simplifying how engineers build, deploy and maintain data-intensive applications 
with the first full-stack data-intensive framework.  

Join our community to keep up with our progress, contribute to igloo or join our team:
{}
---------------------------------------------------------------------------------------

"#,
        style("# Igloo is coming soon").bold(),
        style("https://join.slack.com/t/igloocommunity/shared_invite/zt-25gsnx2x2-9ttVTt4L9LYFrRcM6jimcg").color256(118).bold()
    )
}

#[derive(Debug, Clone)]
pub struct Message {
    pub action: String,
    pub details: String,
}
impl Message {
    pub fn new(action: String, details: String) -> Message {
        Message { action, details }
    }
}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref TERM: Arc<RwLock<CommandTerminal>> =
        Arc::new(RwLock::new(CommandTerminal::new()));
}

macro_rules! show_message {
    ($message_type:expr, $message:expr) => {
        use crate::cli::display::styled_banner;
        use crate::cli::display::TERM;
        use console::{pad_str, style};

        let padder = 14;
        let mut command_terminal = TERM.write().unwrap();

        match $message_type {
            MessageType::Info => {
                command_terminal
                    .term
                    .write_line(&format!(
                        "{} {}",
                        style(pad_str(
                            $message.action.as_str(),
                            padder,
                            console::Alignment::Right,
                            Some("...")
                        ))
                        .blue()
                        .bold(),
                        $message.details
                    ))
                    .expect("failed to write message to terminal");
                command_terminal.counter += 1;
            }
            MessageType::Success => {
                command_terminal
                    .term
                    .write_line(&format!(
                        "{} {}",
                        style(pad_str(
                            $message.action.as_str(),
                            padder,
                            console::Alignment::Right,
                            Some("...")
                        ))
                        .green()
                        .bold(),
                        $message.details
                    ))
                    .expect("failed to write message to terminal");
                command_terminal.counter += 1;
            }
            MessageType::Error => {
                command_terminal
                    .term
                    .write_line(&format!(
                        "{} {}",
                        style(pad_str(
                            $message.action.as_str(),
                            padder,
                            console::Alignment::Right,
                            Some("...")
                        ))
                        .red()
                        .bold(),
                        $message.details
                    ))
                    .expect("failed to write message to terminal");
                command_terminal.counter += 1;
            }
            MessageType::Banner => {
                command_terminal
                    .term
                    .write_line(&styled_banner())
                    .expect("failed to write message to terminal");
                command_terminal.counter += styled_banner().lines().count();
            }
        };
    };
}
