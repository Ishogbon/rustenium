use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionKey {
	#[serde(rename = "userContext")]
	user_context: Option<String>,
	#[serde(rename = "sourceOrigin")]
	source_origin: Option<String>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

