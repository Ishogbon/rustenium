use serde::{Serialize, Deserialize};

use crate::{browsing_context::types::BrowsingContext, script::types::SharedReference};

use super::types::ElementOrigin;

pub type NoneSourceAction = PauseAction;
pub enum InputCommand {
	PerformActions(PerformActions),
	ReleaseActions(ReleaseActions),
	SetFiles(SetFiles),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformActions {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: PerformActionsParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PerformActionsParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "actions")]
	actions: Vec<SourceActions>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum SourceActions {
	NoneSourceActions(NoneSourceActions),
	KeySourceActions(KeySourceActions),
	PointerSourceActions(PointerSourceActions),
	WheelSourceActions(WheelSourceActions),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoneSourceActions {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "id")]
	id: String,
	#[serde(rename = "actions")]
	actions: Vec<NoneSourceAction>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct KeySourceActions {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "id")]
	id: String,
	#[serde(rename = "actions")]
	actions: Vec<KeySourceAction>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeySourceAction {
	PauseAction(PauseAction),
	KeyDownAction(KeyDownAction),
	KeyUpAction(KeyUpAction),
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PointerSourceActions {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "id")]
	id: String,
	#[serde(rename = "parameters")]
	parameters: Option<PointerParameters>,
	#[serde(rename = "actions")]
	actions: Vec<PointerSourceAction>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PointerParameters {
	#[serde(rename = "pointerType")]
	pointer_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PointerSourceAction {
	PauseAction(PauseAction),
	PointerDownAction(PointerDownAction),
	PointerUpAction(PointerUpAction),
	PointerMoveAction(PointerMoveAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WheelSourceActions {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "id")]
	id: String,
	#[serde(rename = "actions")]
	actions: Vec<WheelSourceAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WheelSourceAction {
	PauseAction(PauseAction),
	WheelScrollAction(WheelScrollAction),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PauseAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "duration")]
	duration: Option<u32>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct KeyDownAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct KeyUpAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PointerUpAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "button")]
	button: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerDownAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "button")]
	button: u32,
	#[serde(flatten)]
	pointerCommonProperties: PointerCommonProperties,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PointerMoveAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "x")]
	x: f64,
	#[serde(rename = "y")]
	y: f64,
	#[serde(rename = "duration")]
	duration: Option<u32>,
	#[serde(rename = "origin")]
	origin: Option<Origin>,
	#[serde(flatten)]
	pointerCommonProperties: PointerCommonProperties,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WheelScrollAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "x")]
	x: i32,
	#[serde(rename = "y")]
	y: i32,
	#[serde(rename = "deltaX")]
	delta_x: i32,
	#[serde(rename = "deltaY")]
	delta_y: i32,
	#[serde(rename = "duration")]
	duration: Option<u32>,
	#[serde(rename = "origin")]
	origin: Option<String>,
}

fn pointer_common_properties_default_width() -> u32 {
    1
}

fn pointer_common_properties_default_height() -> u32 {
    1
}

fn pointer_common_properties_default_pressure() -> f64 {
    0.0
}

fn pointer_common_properties_default_tangential_pressure() -> f64 {
    0.0
}

fn pointer_common_properties_default_twist() -> u32 {
    0
}

fn pointer_common_properties_default_altitude_angle() -> f64 {
    0.0
}

fn pointer_common_properties_default_azimuth_angle() -> f64 {
    0.0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerCommonProperties {
    #[serde(rename = "width", default = "pointer_common_properties_default_width")]
    pub width: u32,  // js-uint maps to u32 in Rust

    #[serde(rename = "height", default = "pointer_common_properties_default_height")]
    pub height: u32,

    #[serde(rename = "pressure", default = "pointer_common_properties_default_pressure")]
    pub pressure: f64,

    #[serde(rename = "tangentialPressure", default = "pointer_common_properties_default_tangential_pressure")]
    pub tangential_pressure: f64,

    #[serde(rename = "twist", default = "pointer_common_properties_default_twist")]
    pub twist: u32,

    #[serde(rename = "altitudeAngle", default = "pointer_common_properties_default_altitude_angle")]
    pub altitude_angle: f64,

    #[serde(rename = "azimuthAngle", default = "pointer_common_properties_default_azimuth_angle")]
    pub azimuth_angle: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseActions {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ReleaseActionsParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseActionsParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFiles {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: SetFilesParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SetFilesParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "element")]
	element: SharedReference,
	#[serde(rename = "files")]
	files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Origin {
	#[serde(rename = "viewport")]
	Viewport,
	#[serde(rename = "pointer")]
	Pointer,
	Element(ElementOrigin),
}

