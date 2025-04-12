use serde::{Serialize, Deserialize};
use crate::script::types::{Channel, Realm, RealmInfo, RemoteValue, Source};
pub enum ScriptEvent {
	Message(Message),
	RealmCreated(RealmCreated),
	RealmDestroyed(RealmDestroyed),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: MessageParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct MessageParameters {
	#[serde(rename = "channel")]
	channel: Channel,
	#[serde(rename = "data")]
	data: RemoteValue,
	#[serde(rename = "source")]
	source: Source,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmCreated {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: RealmInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmDestroyed {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: RealmDestroyedParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct RealmDestroyedParameters {
	#[serde(rename = "realm")]
	realm: Realm,
}

