#[derive(Debug, Serialize, Deserialize)]
pub struct FileDialogOpened {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: FileDialogInfo,
}

pub struct FileDialogInfo {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "element")]
	element: Option<SharedReference>,
	#[serde(rename = "multiple")]
	multiple: bool,
}

