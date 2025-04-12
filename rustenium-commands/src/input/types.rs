pub struct ElementOrigin {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "element")]
	element: SharedReference,
}

