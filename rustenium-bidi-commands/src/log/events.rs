use serde::{Serialize, Deserialize};

use super::types::Entry;

#[derive(Debug, Serialize, Deserialize)]
enum EntryAddedMethod {
	#[serde(rename = "log.entryAdded")]
	EntryAdded,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryAdded {
	#[serde(rename = "method")]
	pub method: EntryAddedMethod,
	#[serde(rename = "params")]
	pub params: Entry,
}

