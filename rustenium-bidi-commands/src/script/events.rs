use serde::{Serialize, Deserialize};
use crate::script::types::{Channel, Realm, RealmInfo, RemoteValue, Source};

pub enum ScriptEvent {
	Message(Message),
	RealmCreated(RealmCreated),
	RealmDestroyed(RealmDestroyed),
}

#[derive(Debug, Serialize, Deserialize)]
enum MessageMethod {
	#[serde(rename = "script.message")]
	Message,
}

#[derive(Debug, Serialize, Deserialize)]
enum RealmCreatedMethod {
	#[serde(rename = "script.realmCreated")]
	RealmCreated,
}

#[derive(Debug, Serialize, Deserialize)]
enum RealmDestroyedMethod {
	#[serde(rename = "script.realmDestroyed")]
	RealmDestroyed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
	#[serde(rename = "method")]
	pub method: MessageMethod,
	#[serde(rename = "params")]
	pub params: MessageParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageParameters {
	#[serde(rename = "channel")]
	pub channel: Channel,
	#[serde(rename = "data")]
	pub data: RemoteValue,
	#[serde(rename = "source")]
	pub source: Source,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmCreated {
	#[serde(rename = "method")]
	pub method: RealmCreatedMethod,
	#[serde(rename = "params")]
	pub params: RealmInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmDestroyed {
	#[serde(rename = "method")]
	pub method: RealmDestroyedMethod,
	#[serde(rename = "params")]
	pub params: RealmDestroyedParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmDestroyedParameters {
	#[serde(rename = "realm")]
	pub realm: Realm,
}

