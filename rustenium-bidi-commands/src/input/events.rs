use serde::{Serialize, Deserialize};

use crate::browsing_context::types::BrowsingContext;
use crate::script::types::SharedReference;

#[derive(Debug, Serialize, Deserialize)]
pub enum InputEvent {
	FileDialogOpened(FileDialogOpened),
}

#[derive(Debug, Serialize, Deserialize)]
enum FileDialogOpenedMethod {
	InputFileDialogOpened,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDialogOpened {
	#[serde(rename = "method")]
	pub method: FileDialogOpenedMethod,
	#[serde(rename = "params")]
	pub params: FileDialogInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDialogInfo {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "element")]
	pub element: Option<SharedReference>,
	#[serde(rename = "multiple")]
	pub multiple: bool,
}

