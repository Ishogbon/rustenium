use browsing_context::events::BrowsingContextEvent;
use input::events::InputEvent;
use log::events::LogEvent;
use network::events::NetworkEvent;
use script::events::ScriptEvent;
use serde::{Deserialize, Serialize, Deserializer};
use crate::browser::commands::BrowserCommand;
use crate::browsing_context::commands::{BrowsingContextCommand, BrowsingContextResult};
use crate::emulation::commands::EmulationCommand;
use crate::input::commands::InputCommand;
use crate::network::commands::{NetworkCommand, NetworkResult};
use crate::script::commands::{ScriptCommand, ScriptResult};
use crate::session::commands::{SessionCommand, SessionResult};
use crate::storage::commands::{StorageCommand, StorageResult};
use crate::web_extension::commands::{WebExtensionCommand, WebExtensionResult};

pub mod browser;
pub mod session;
pub mod browsing_context;
pub mod script;
pub mod emulation;
pub mod input;
pub mod log;
pub mod network;
pub mod storage;
pub mod web_extension;

fn float_or_int_to_i32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_i64() {
                Ok(i as u32)
            } else if let Some(f) = num.as_f64() {
                Ok(f as u32)
            } else {
                Err(serde::de::Error::custom("Invalid number"))
            }
        }
        _ => Err(serde::de::Error::custom("Expected a number")),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub id: u32,
    #[serde(flatten)]
    pub command_data: CommandData,
    #[serde(rename = "extension")]
    pub extension: Option<serde_cbor::Value>
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandData {
    BrowserCommand(BrowserCommand),
    BrowsingContextCommand(BrowsingContextCommand),
    EmulationCommand(EmulationCommand),
    InputCommand(InputCommand),
    NetworkCommand(NetworkCommand),
    ScriptCommand(ScriptCommand),
    SessionCommand(SessionCommand),
    StorageCommand(StorageCommand),
    WebExtensionCommand(WebExtensionCommand),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResult {
    #[serde(rename = "extension")]
    extension: Option<serde_cbor::Value>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandResult {
    BrowsingContextResult(BrowsingContextResult),
    NetworkResult(NetworkResult),
    ScriptResult(ScriptResult),
    SessionResult(SessionResult),
    StorageResult(StorageResult),
    WebExtensionResult(WebExtensionResult),
    EmptyResult(EmptyResult),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandResponseType {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorResponseType {
    #[serde(rename = "error")]
    Error
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResponse {
    #[serde(rename = "type")]
    pub r#type: CommandResponseType,
    #[serde(deserialize_with = "float_or_int_to_i32", rename = "id")]
    pub id: u32,
    #[serde(rename = "result")]
    pub result: CommandResult,
    #[serde(rename = "extension")]
    pub extension: Option<serde_cbor::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event{
    #[serde(rename = "type")]
    pub r#type: EventType,
    #[serde(flatten)]
    pub event_data: EventData,
    #[serde(rename = "extension")]
    pub extension: Option<serde_cbor::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EventData {
    BrowsingContextEvent(BrowsingContextEvent),
    InputEvent(InputEvent),
    LogEvent(LogEvent),
    NetworkEvent(NetworkEvent),
    ScriptEvent(ScriptEvent)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "invalid argument")]
    InvalidArgument,
    #[serde(rename = "invalid selector")]
    InvalidSelector,
    #[serde(rename = "invalid session id")]
    InvalidSessionId,
    #[serde(rename = "invalid web extension")]
    InvalidWebExtension,
    #[serde(rename = "move target out of bounds")]
    MoveTargetOutOfBounds,
    #[serde(rename = "no such alert")]
    NoSuchAlert,
    #[serde(rename = "no such element")]
    NoSuchElement,
    #[serde(rename = "no such frame")]
    NoSuchFrame,
    #[serde(rename = "no such handle")]
    NoSuchHandle,
    #[serde(rename = "no such history entry")]
    NoSuchHistoryEntry,
    #[serde(rename = "no such intercept")]
    NoSuchIntercept,
    #[serde(rename = "no such node")]
    NoSuchNode,
    #[serde(rename = "no such request")]
    NoSuchRequest,
    #[serde(rename = "no such script")]
    NoSuchScript,
    #[serde(rename = "no such storage partition")]
    NoSuchStoragePartition,
    #[serde(rename = "no such user context")]
    NoSuchUserContext,
    #[serde(rename = "no such web extension")]
    NoSuchWebExtension,
    #[serde(rename = "session not created")]
    SessionNotCreated,
    #[serde(rename = "unable to capture screen")]
    UnableToCaptureScreen,
    #[serde(rename = "unable to close browser")]
    UnableToCloseBrowser,
    #[serde(rename = "unable to set cookie")]
    UnableToSetCookie,
    #[serde(rename = "unable to set file input")]
    UnableToSetFileInput,
    #[serde(rename = "underspecified storage partition")]
    UnderspecifiedStoragePartition,
    #[serde(rename = "unknown command")]
    UnknownCommand,
    #[serde(rename = "unknown error")]
    UnknownError,
    #[serde(rename = "unsupported operation")]
    UnsupportedOperation,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::InvalidArgument => write!(f, "invalid argument"),
            ErrorCode::InvalidSelector => write!(f, "invalid selector"),
            ErrorCode::InvalidSessionId => write!(f, "invalid session id"),
            ErrorCode::InvalidWebExtension => write!(f, "invalid web extension"),
            ErrorCode::MoveTargetOutOfBounds => write!(f, "move target out of bounds"),
            ErrorCode::NoSuchAlert => write!(f, "no such alert"),
            ErrorCode::NoSuchElement => write!(f, "no such element"),
            ErrorCode::NoSuchFrame => write!(f, "no such frame"),
            ErrorCode::NoSuchHandle => write!(f, "no such handle"),
            ErrorCode::NoSuchHistoryEntry => write!(f, "no such history entry"),
            ErrorCode::NoSuchIntercept => write!(f, "no such intercept"),
            ErrorCode::NoSuchNode => write!(f, "no such node"),
            ErrorCode::NoSuchRequest => write!(f, "no such request"),
            ErrorCode::NoSuchScript => write!(f, "no such script"),
            ErrorCode::NoSuchStoragePartition => write!(f, "no such storage partition"),
            ErrorCode::NoSuchUserContext => write!(f, "no such user context"),
            ErrorCode::NoSuchWebExtension => write!(f, "no such web extension"),
            ErrorCode::SessionNotCreated => write!(f, "session not created"),
            ErrorCode::UnableToCaptureScreen => write!(f, "unable to capture screen"),
            ErrorCode::UnableToCloseBrowser => write!(f, "unable to close browser"),
            ErrorCode::UnableToSetCookie => write!(f, "unable to set cookie"),
            ErrorCode::UnableToSetFileInput => write!(f, "unable to set file input"),
            ErrorCode::UnderspecifiedStoragePartition => write!(f, "underspecified storage partition"),
            ErrorCode::UnknownCommand => write!(f, "unknown command"),
            ErrorCode::UnknownError => write!(f, "unknown error"),
            ErrorCode::UnsupportedOperation => write!(f, "unsupported operation"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub r#type: ErrorResponseType,
    #[serde(rename = "id")]
    pub id: Option<u32>,
    #[serde(rename = "error")]
    pub error: ErrorCode,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "stacktrace")]
    pub stacktrace: Option<String>,
    #[serde(rename = "extension")]
    pub extension: Option<serde_cbor::Value>
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error[{}]: {} (ID: {}){}", 
            self.error,
            self.message,
            self.id.map_or("None".to_string(), |id| id.to_string()),
            self.stacktrace.as_ref().map_or("".to_string(), |st| format!("\nStacktrace:\n{}", st))
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    CommandResponse(CommandResponse),
    ErrorResponse(ErrorResponse),
    Event(Event),
}

#[cfg(test)]
mod tests {
    use super::*;
}
