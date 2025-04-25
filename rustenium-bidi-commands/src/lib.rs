use browsing_context::events::BrowsingContextEvent;
use input::events::InputEvent;
use log::events::LogEvent;
use network::events::NetworkEvent;
use script::events::ScriptEvent;
use serde::{Deserialize, Serialize};
use crate::browser::commands::BrowserCommand;
use crate::browsing_context::commands::BrowsingContextResult;
use crate::emulation::commands::EmulationCommand;
use crate::input::commands::InputCommand;
use crate::network::commands::{NetworkCommand, NetworkResult};
use crate::script::commands::{ScriptCommand, ScriptResult};
use crate::session::commands::{SessionCommand, SessionResult};
use crate::storage::commands::{StorageCommand, StorageResult};
use crate::web_extension::commands::WebExtensionResult;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub id: u32,
    #[serde(flatten)]
    pub command_data: CommandData,
    #[serde(rename = "extension")]
    pub extension: Option<serde_cbor::Value>
}


#[derive(Debug, Serialize, Deserialize)]
pub enum CommandData {
    BrowserCommand(BrowserCommand),
    BrowsingContextCommand(BrowserCommand),
    EmulationCommand(EmulationCommand),
    InputCommand(InputCommand),
    NetworkCommand(NetworkCommand),
    ScriptCommand(ScriptCommand),
    SessionCommand(SessionCommand),
    StorageCommand(StorageCommand),
    WebExtensionCommand(WebExtensionResult),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResult {
    #[serde(rename = "extension")]
    extension: Option<serde_cbor::Value>
}

#[derive(Debug, Serialize, Deserialize)]
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
    #[serde(rename = "id")]
    pub id: i32,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    r#type: ErrorResponseType,
    #[serde(rename = "id")]
    id: Option<u32>,
    #[serde(rename = "error")]
    error: ErrorCode,
    #[serde(rename = "message")]
    message: String,
    #[serde(rename = "stacktrace")]
    stacktrace: Option<String>,
    #[serde(rename = "extension")]
    extension: Option<serde_cbor::Value>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    CommandResponse(CommandResponse),
    ErrorResponse(ErrorResponse),
    Event(Event),
}

#[cfg(test)]
mod tests {
    use super::*;
}
