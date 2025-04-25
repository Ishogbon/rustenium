use serde::{Serialize, Deserialize};

use super::types::{CapabilitiesRequest, ProxyConfiguration, SubscriptionRequest, UnsubscribeByAttributesRequest, UnsubscribeByIDRequest, UserPromptHandler};

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionCommand {
	End(End),
	New(New),
	Status(Status),
	Subscribe(Subscribe),
	Unsubscribe(Unsubscribe),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SessionResult {
	NewResult(NewResult),
	StatusResult(StatusResult),
	SubscribeResult(SubscribeResult),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewResult {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "capabilities")]
    pub capabilities: Capabilities,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Capabilities {
    #[serde(rename = "acceptInsecureCerts")]
    pub accept_insecure_certs: bool,
    #[serde(rename = "browserName")]
    pub browser_name: String,
    #[serde(rename = "browserVersion")]
    pub browser_version: String,
    #[serde(rename = "platformName")]
    pub platform_name: String,
    #[serde(rename = "setWindowRect")]
    pub set_window_rect: bool,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[serde(rename = "proxy")]
    pub proxy: Option<ProxyConfiguration>,
    #[serde(rename = "unhandledPromptBehavior")]
    pub unhandled_prompt_behavior: Option<UserPromptHandler>,
    #[serde(rename = "webSocketUrl")]
    pub web_socket_url: Option<String>,
    #[serde(flatten)]
    pub extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusResult {
    #[serde(rename = "ready")]
    pub ready: bool,
    #[serde(rename = "message")]
    pub message: String,  // 'text' type maps to String in Rust
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubscribeResult {
    #[serde(rename = "subscription")]
    pub subscription: String,  // Based on usage in UnsubscribeByIDRequest, Subscription appears to be a String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusMethod {
    #[serde(rename = "session.status")]
    SessionStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "method")]
    pub method: StatusMethod,
    #[serde(rename = "params")]
    pub params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum NewMethod {
    #[serde(rename = "session.new")]
    SessionNew,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct New {
    #[serde(rename = "method")]
    pub method: NewMethod,
    #[serde(rename = "params")]
    pub params: NewParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewParameters {
	#[serde(rename = "capabilities")]
	pub capabilities: CapabilitiesRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EndMethod {
    #[serde(rename = "session.end")]
    SessionEnd,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct End {
    #[serde(rename = "method")]
    pub method: EndMethod,
    #[serde(rename = "params")]
    pub params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeMethod {
    #[serde(rename = "session.subscribe")]
    SessionSubscribe,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    #[serde(rename = "method")]
    pub method: SubscribeMethod,
    #[serde(rename = "params")]
    pub params: SubscriptionRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UnsubscribeMethod {
    #[serde(rename = "session.unsubscribe")]
    SessionUnsubscribe,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unsubscribe {
    #[serde(rename = "method")]
    pub method: UnsubscribeMethod,

    #[serde(rename = "params")]
    pub params: UnsubscribeParameters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnsubscribeParameters {
    ByAttributes(UnsubscribeByAttributesRequest),
    ById(UnsubscribeByIDRequest),
}