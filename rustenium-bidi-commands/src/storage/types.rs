use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionKey {
	#[serde(rename = "userContext")]
	pub user_context: Option<String>,
	#[serde(rename = "sourceOrigin")]
	pub source_origin: Option<String>,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

