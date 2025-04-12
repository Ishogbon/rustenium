use serde::{Serialize, Deserialize};

use crate::script::types::SharedReference;

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementOrigin {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "element")]
	element: SharedReference,
}

