use serde::{Serialize, Deserialize};

use crate::script::types::SharedReference;

#[derive(Debug, Serialize, Deserialize)]
enum ElementOriginType {
	#[serde(rename = "element")]
	Element,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementOrigin {
	#[serde(rename = "type")]
	pub r#type: ElementOriginType,
	#[serde(rename = "element")]
	pub element: SharedReference,
}

