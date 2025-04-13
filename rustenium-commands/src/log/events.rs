use serde::{Serialize, Deserialize};

use super::types::Entry;

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryAdded {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: Entry,
}

