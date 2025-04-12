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

pub struct PerformActionsParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "actions")]
	actions: Vec<SourceActions>,
}

pub enum SourceActions {
	NoneSourceActions(NoneSourceActions),
	KeySourceActions(KeySourceActions),
	PointerSourceActions(PointerSourceActions),
	WheelSourceActions(WheelSourceActions),
}

pub struct NoneSourceActions {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "id")]
	id: String,
	#[serde(rename = "actions")]
	actions: Vec<NoneSourceAction>,
}

pub struct KeySourceActions {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "id")]
	id: String,
	#[serde(rename = "actions")]
	actions: Vec<KeySourceAction>,
}

pub enum KeySourceAction {
	PauseAction(PauseAction),
	KeyDownAction(KeyDownAction),
	KeyUpAction(KeyUpAction),
}

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

pub struct PointerParameters {
	#[serde(rename = "pointerType")]
	pointer_type: Option<String>,
}

pub enum PointerSourceAction {
	PauseAction(PauseAction),
	PointerDownAction(PointerDownAction),
	PointerUpAction(PointerUpAction),
	PointerMoveAction(PointerMoveAction),
}

pub struct WheelSourceActions {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "id")]
	id: String,
	#[serde(rename = "actions")]
	actions: Vec<WheelSourceAction>,
}

pub enum WheelSourceAction {
	PauseAction(PauseAction),
	WheelScrollAction(WheelScrollAction),
}

pub struct PauseAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "duration")]
	duration: Option<u32>,
}

pub struct KeyDownAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
}

pub struct KeyUpAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "value")]
	value: String,
}

pub struct PointerUpAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "button")]
	button: u32,
}

pub struct PointerDownAction {
	#[serde(rename = "type")]
	r#type: String,
	#[serde(rename = "button")]
	button: u32,
	PointerCommonProperties(PointerCommonProperties),
}

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
	PointerCommonProperties(PointerCommonProperties),
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct PointerCommonProperties {
	#[serde(rename = "width")]
	width: Option<>,
	#[serde(rename = "height")]
	height: Option<>,
	#[serde(rename = "pressure")]
	pressure: Option<>,
	#[serde(rename = "tangentialPressure")]
	tangential_pressure: Option<>,
	#[serde(rename = "twist")]
	twist: Option<>,
	#[serde(rename = "PI")]
	p_i: PI,
	#[serde(rename = "altitudeAngle")]
	altitude_angle: Option<>,
	#[serde(rename = "PI")]
	p_i: PI,
	#[serde(rename = "azimuthAngle")]
	azimuth_angle: Option<>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseActions {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ReleaseActionsParameters,
}

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

pub struct SetFilesParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "element")]
	element: SharedReference,
	#[serde(rename = "files")]
	files: Vec<String>,
}

