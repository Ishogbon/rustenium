use serde::{Deserialize, Serialize};

use super::types::{BaseParameters, Initiator, ResponseData};

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkEvent {
	AuthRequired(AuthRequired),
	BeforeRequestSent(BeforeRequestSent),
	FetchError(FetchError),
	ResponseCompleted(ResponseCompleted),
	ResponseStarted(ResponseStarted),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthRequiredMethod {
	#[serde(rename = "network.authRequired")]
	NetworkAuthRequired,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequired {
	#[serde(rename = "method")]
	pub method: AuthRequiredMethod,
	#[serde(rename = "params")]
	pub params: AuthRequiredParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequiredParameters {
	#[serde(flatten)]
	pub base: BaseParameters,
	#[serde(rename = "response")]
	pub response: ResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BeforeRequestSentMethod {
	#[serde(rename = "network.beforeRequestSent")]
	NetworkBeforeRequestSent,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeforeRequestSentParameters {
	#[serde(flatten)]
	pub base: BaseParameters,
	#[serde(skip_serializing_if = "Option::is_none", rename = "initiator")]
	pub initiator: Option<Initiator>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeforeRequestSent {
	#[serde(rename = "method")]
	pub method: BeforeRequestSentMethod,

	#[serde(rename = "params")]
	pub params: BeforeRequestSentParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FetchErrorMethod {
	#[serde(rename = "network.fetchError")]
	FetchError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchErrorParameters {
	#[serde(flatten)]
	pub base: BaseParameters,

	#[serde(rename = "errorText")]
	pub error_text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchError {
	#[serde(rename = "method")]
	pub method: FetchErrorMethod,

	#[serde(rename = "params")]
	pub params: FetchErrorParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseCompletedMethod {
	#[serde(rename = "network.responseCompleted")]
	NetworkResponseCompleted,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCompletedParameters {
	#[serde(flatten)]
	pub base: BaseParameters,

	#[serde(rename = "response")]
	pub response: ResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseCompleted {
	#[serde(rename = "method")]
	pub method: ResponseCompletedMethod,

	#[serde(rename = "params")]
	pub params: ResponseCompletedParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseStartedMethod {
	#[serde(rename = "network.responseStarted")]
	NetworkResponseStarted,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseStartedParameters {
	#[serde(flatten)]
	pub base: BaseParameters,

	#[serde(rename = "response")]
	pub response: ResponseData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseStarted {
	#[serde(rename = "method")]
	pub method: ResponseStartedMethod,

	#[serde(rename = "params")]
	pub params: ResponseStartedParameters,
}
