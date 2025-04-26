use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct PartitionKey {
	#[serde(skip_serializing_if = "Option::is_none", rename = "userContext")]
	pub user_context: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "sourceOrigin")]
	pub source_origin: Option<String>,
	#[serde(flatten)]
	pub extension: Option<serde_cbor::Value>,
}

