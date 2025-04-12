pub struct ElementOrigin {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "element")]
	element: SharedReference,
}

