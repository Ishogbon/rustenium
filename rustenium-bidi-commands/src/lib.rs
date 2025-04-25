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
#[cfg(test)]
mod tests {
    use super::*;
}
