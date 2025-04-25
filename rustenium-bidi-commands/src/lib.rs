use serde::{Deserialize, Serialize};
use crate::browsing_context::commands::BrowsingContextResult;
use crate::network::commands::NetworkResult;
use crate::script::commands::ScriptResult;
use crate::session::commands::SessionResult;
use crate::storage::commands::StorageResult;
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
pub struct EmptyResult {
    #[serde(rename = "extension")]
    extension: Option<serde_cbor::Value>
}

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

#[cfg(test)]
mod tests {
    use super::*;
}
