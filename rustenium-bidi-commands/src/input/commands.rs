use serde::{Serialize, Deserialize};

use crate::{browsing_context::types::BrowsingContext, script::types::SharedReference};

use super::types::ElementOrigin;

pub type NoneSourceAction = PauseAction;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputCommand {
	PerformActions(PerformActions),
	ReleaseActions(ReleaseActions),
	SetFiles(SetFiles),
}

#[derive(Debug, Serialize, Deserialize)]
enum PerformActionsMethod {
	#[serde(rename = "input.performActions")]
	InputPerformActions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PerformActions {
	#[serde(rename = "method")]
	pub method: PerformActionsMethod,
	#[serde(rename = "params")]
	pub params: PerformActionsParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PerformActionsParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "actions")]
	pub actions: Vec<SourceActions>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum SourceActions {
	NoneSourceActions(NoneSourceActions),
	KeySourceActions(KeySourceActions),
	PointerSourceActions(PointerSourceActions),
	WheelSourceActions(WheelSourceActions),
}

#[derive(Debug, Serialize, Deserialize)]
enum NoneSourceActionsType {
	#[serde(rename = "none")]
	None,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoneSourceActions {
	#[serde(rename = "type")]
	pub r#type: NoneSourceActionsType,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "actions")]
	pub actions: Vec<NoneSourceAction>,
}


#[derive(Debug, Serialize, Deserialize)]
enum KeySourceActionsType {
	#[serde(rename = "key")]
	Key,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeySourceActions {
	#[serde(rename = "type")]
	pub r#type: KeySourceActionsType,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "actions")]
	pub actions: Vec<KeySourceAction>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeySourceAction {
	PauseAction(PauseAction),
	KeyDownAction(KeyDownAction),
	KeyUpAction(KeyUpAction),
}


#[derive(Debug, Serialize, Deserialize)]
enum PointerSourceActionsType {
	#[serde(rename = "pointer")]
	Pointer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerSourceActions {
	#[serde(rename = "type")]
	pub r#type: PointerSourceActionsType,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "parameters")]
	pub parameters: Option<PointerParameters>,
	#[serde(rename = "actions")]
	pub actions: Vec<PointerSourceAction>,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum PointerType {
    #[serde(rename = "mouse")]
    Mouse,
    #[serde(rename = "pen")]
    Pen,
    #[serde(rename = "touch")]
    Touch,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerParameters {
    #[serde(rename = "pointerType", default = "pointer_parameters_default_pointer_type")]
    pointer_type: PointerType,
}

fn pointer_parameters_default_pointer_type() -> PointerType {
    PointerType::Mouse
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PointerSourceAction {
	PauseAction(PauseAction),
	PointerDownAction(PointerDownAction),
	PointerUpAction(PointerUpAction),
	PointerMoveAction(PointerMoveAction),
}

#[derive(Debug, Serialize, Deserialize)]
enum WheelSourceActionsType {
	#[serde(rename = "wheel")]
	Wheel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WheelSourceActions {
	#[serde(rename = "type")]
	pub r#type: WheelSourceActionsType,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "actions")]
	pub actions: Vec<WheelSourceAction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WheelSourceAction {
	PauseAction(PauseAction),
	WheelScrollAction(WheelScrollAction),
}

#[derive(Debug, Serialize, Deserialize)]
enum PauseActionType {
	#[serde(rename = "pause")]
	Pause,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PauseAction {
	#[serde(rename = "type")]
	pub r#type: PauseActionType,
	#[serde(rename = "duration")]
	pub duration: Option<u32>,
}


#[derive(Debug, Serialize, Deserialize)]
enum KeyDownActionType {
	#[serde(rename = "keyDown")]
	KeyDown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyDownAction {
	#[serde(rename = "type")]
	pub r#type: KeyDownActionType,
	#[serde(rename = "value")]
	pub value: String,
}


#[derive(Debug, Serialize, Deserialize)]
enum KeyUpActionType {
	#[serde(rename = "keyUp")]
	KeyUp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyUpAction {
	#[serde(rename = "type")]
	pub r#type: KeyUpActionType,
	#[serde(rename = "value")]
	pub value: String,
}


#[derive(Debug, Serialize, Deserialize)]
enum PointerUpActionType {
	#[serde(rename = "pointerUp")]
	PointerUp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerUpAction {
	#[serde(rename = "type")]
	pub r#type: PointerUpActionType,
	#[serde(rename = "button")]
	pub button: u32,
}

#[derive(Debug, Serialize, Deserialize)]
enum PointerDownActionType {
	#[serde(rename = "pointerDown")]
	PointerDown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerDownAction {
	#[serde(rename = "type")]
	pub r#type: PointerDownActionType,
	#[serde(rename = "button")]
	pub button: u32,
	#[serde(flatten)]
	pub extension: PointerCommonProperties,
}


#[derive(Debug, Serialize, Deserialize)]
enum PointerMoveActionType {
	#[serde(rename = "pointerMove")]
	PointerMove,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerMoveAction {
	#[serde(rename = "type")]
	pub r#type: PointerMoveActionType,
	#[serde(rename = "x")]
	pub x: f64,
	#[serde(rename = "y")]
	pub y: f64,
	#[serde(rename = "duration")]
	pub duration: Option<u32>,
	#[serde(rename = "origin")]
	pub origin: Option<Origin>,
	#[serde(flatten)]
	pub extension: PointerCommonProperties,
}


#[derive(Debug, Serialize, Deserialize)]
enum WheelScrollActionType {
	#[serde(rename = "scroll")]
	Scroll,
}

fn wheel_scroll_action_default_origin() -> Origin {
	Origin::Viewport
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WheelScrollAction {
	#[serde(rename = "type")]
	pub r#type: WheelScrollActionType,
	#[serde(rename = "x")]
	pub x: i32,
	#[serde(rename = "y")]
	pub y: i32,
	#[serde(rename = "deltaX")]
	pub delta_x: i32,
	#[serde(rename = "deltaY")]
	pub delta_y: i32,
	#[serde(rename = "duration")]
	pub duration: Option<u32>,
	#[serde(rename = "origin", default = "wheel_scroll_action_default_origin")]
	pub origin: Origin,
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
    pub width: u32,

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
enum ReleaseActionsMethod {
	#[serde(rename = "input.releaseActions")]
	InputReleaseActions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseActions {
	#[serde(rename = "method")]
	pub method: ReleaseActionsMethod,
	#[serde(rename = "params")]
	pub params: ReleaseActionsParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseActionsParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
enum SetFilesMethod {
	#[serde(rename = "input.setFiles")]
	InputSetFiles,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFiles {
	#[serde(rename = "method")]
	pub method: SetFilesMethod,
	#[serde(rename = "params")]
	pub params: SetFilesParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct SetFilesParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "element")]
	pub element: SharedReference,
	#[serde(rename = "files")]
	pub files: Vec<String>,
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

