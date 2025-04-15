use serde::{Serialize, Deserialize};

use crate::browsing_context::types::BrowsingContext;
use crate::script::types::SharedReference;

#[derive(Debug, Serialize, Deserialize)]
enum FileDialogOpenedMethod {
	#[serde(rename = "input.fileDialogOpened")]
	InputFileDialogOpened,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDialogOpened {
	#[serde(rename = "method")]
	method: FileDialogOpenedMethod,
	#[serde(rename = "params")]
	params: FileDialogInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDialogInfo {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "element")]
	element: Option<SharedReference>,
	#[serde(rename = "multiple")]
	multiple: bool,
}

