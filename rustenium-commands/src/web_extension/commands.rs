use serde::{Deserialize, Serialize};

pub type Extension = String;

pub enum WebExtensionCommand {
	Install(Install),
	Uninstall(Uninstall),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallResult {
	#[serde(rename = "extension")]
	pub extension: Extension,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Install {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: InstallParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstallParameters {
	#[serde(rename = "extensionData")]
	extension_data: ExtensionData,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ExtensionData {
	ExtensionArchivePath(ExtensionArchivePath),
	ExtensionBase64Encoded(ExtensionBase64Encoded),
	ExtensionPath(ExtensionPath),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionPath {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "path")]
	path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionArchivePath {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "path")]
	path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionBase64Encoded {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Uninstall {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: UninstallParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UninstallParameters {
	#[serde(rename = "extension")]
	extension: Extension,
}
