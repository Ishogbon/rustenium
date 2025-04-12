pub enum WebExtensionCommand {
	Install(Install),
	Uninstall(Uninstall),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Install {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: InstallParameters,
}

pub struct InstallParameters {
	#[serde(rename = "extensionData")]
	extension_data: ExtensionData,
}

pub enum ExtensionData {
	ExtensionArchivePath(ExtensionArchivePath),
	ExtensionBase64Encoded(ExtensionBase64Encoded),
	ExtensionPath(ExtensionPath),
}

pub struct ExtensionPath {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "path")]
	path: String,
}

pub struct ExtensionArchivePath {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "path")]
	path: String,
}

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

pub struct UninstallParameters {
	#[serde(rename = "extension")]
	extension: Extension,
}

