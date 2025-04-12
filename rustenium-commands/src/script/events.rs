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

pub struct RealmDestroyedParameters {
	#[serde(rename = "realm")]
	realm: Realm,
}

