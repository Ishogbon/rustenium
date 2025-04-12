use serde::{Serialize, Deserialize};
pub enum SessionCommand {
	End(End),
	New(New),
	Status(Status),
	Subscribe(Subscribe),
	Unsubscribe(Unsubscribe),
}

pub enum SessionResult {
	NewResult(NewResult),
	StatusResult(StatusResult),
	SubscribeResult(SubscribeResult),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct New {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: NewParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewParameters {
	#[serde(rename = "capabilities")]
	capabilities: CapabilitiesRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct End {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: SubscriptionRequest,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unsubscribe {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: UnsubscribeParameters,
}

