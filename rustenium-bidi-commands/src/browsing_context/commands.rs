use serde::{Serialize, Deserialize};
use crate::{browser::types::UserContext, browsing_context::types::BrowsingContext, script::types::{NodeRemoteValue, SerializationOptions, SharedReference}};

use super::types::{InfoList, Locator, Navigation};


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextCommand {
	Activate(Activate),
	CaptureScreenshot(CaptureScreenshot),
	Close(Close),
	Create(Create),
	GetTree(GetTree),
	HandleUserPrompt(HandleUserPrompt),
	LocateNodes(LocateNodes),
	Navigate(Navigate),
	Print(Print),
	Reload(Reload),
	SetViewport(SetViewport),
	TraverseHistory(TraverseHistory),
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextResult {
	CaptureScreenshotResult(CaptureScreenshotResult),
	CreateResult(CreateResult),
	GetTreeResult(GetTreeResult),
	LocateNodesResult(LocateNodesResult),
	NavigateResult(NavigateResult),
	PrintResult(PrintResult),
	TraverseHistoryResult(TraverseHistoryResult),
}

#[derive(Debug, Serialize, Deserialize)]
enum ActivateMethod {
	#[serde(rename = "browsingContext.activate")]
	BrowsingContextActivate,
}

#[derive(Debug, Serialize, Deserialize)]
enum CaptureScreenshotMethod {
	#[serde(rename = "browsingContext.captureScreenshot")]
	BrowsingContextCaptureScreenshot,
}

#[derive(Debug, Serialize, Deserialize)]
enum CloseMethod {
	#[serde(rename = "browsingContext.close")]
	BrowsingContextClose,
}

#[derive(Debug, Serialize, Deserialize)]
enum CreateMethod {
	#[serde(rename = "browsingContext.create")]
	BrowsingContextCreate,
}

#[derive(Debug, Serialize, Deserialize)]
enum GetTreeMethod {
	#[serde(rename = "browsingContext.getTree")]
	BrowsingContextGetTree,
}

#[derive(Debug, Serialize, Deserialize)]
enum HandleUserPromptMethod {
	#[serde(rename = "browsingContext.handleUserPrompt")]
	BrowsingContextHandleUserPrompt,
}

#[derive(Debug, Serialize, Deserialize)]
enum LocateNodesMethod {
	#[serde(rename = "browsingContext.locateNodes")]
	BrowsingContextLocateNodes,
}

#[derive(Debug, Serialize, Deserialize)]
enum NavigateMethod {
	#[serde(rename = "browsingContext.navigate")]
	BrowsingContextNavigate,
}

#[derive(Debug, Serialize, Deserialize)]
enum PrintMethod {
	#[serde(rename = "browsingContext.print")]
	BrowsingContextPrint,
}

#[derive(Debug, Serialize, Deserialize)]
enum ReloadMethod {
	#[serde(rename = "browsingContext.reload")]
	BrowsingContextReload,
}

#[derive(Debug, Serialize, Deserialize)]
enum SetViewportMethod {
	#[serde(rename = "browsingContext.setViewport")]
	BrowsingContextSetViewport,
}

#[derive(Debug, Serialize, Deserialize)]
enum TraverseHistoryMethod {
	#[serde(rename = "browsingContext.traverseHistory")]
	BrowsingContextTraverseHistory,
}

// For the clip rectangle types
#[derive(Debug, Serialize, Deserialize)]
enum ElementClipRectangleType {
	#[serde(rename = "element")]
	Element,
}

#[derive(Debug, Serialize, Deserialize)]
enum BoxClipRectangleType {
	#[serde(rename = "box")]
	Box,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Activate {
	#[serde(rename = "method")]
	pub method: ActivateMethod,
	#[serde(rename = "params")]
	pub params: ActivateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivateParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CaptureScreenshotParametersOrigin {
	Viewport,
	Document,
}

fn capture_screenshot_parameters_default_origin() -> CaptureScreenshotParametersOrigin {
	CaptureScreenshotParametersOrigin::Viewport
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureScreenshot {
	#[serde(rename = "method")]
	pub method: CaptureScreenshotMethod,
	#[serde(rename = "params")]
	pub params: CaptureScreenshotParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureScreenshotParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "origin", default = "capture_screenshot_parameters_default_origin")]
	pub origin: CaptureScreenshotParametersOrigin,
	#[serde(skip_serializing_if = "Option::is_none", rename = "format")]
	pub format: Option<ImageFormat>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "clip")]
	pub clip: Option<ClipRectangle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageFormat {
	#[serde(rename = "type")]
	pub r#type: String,
	#[serde(skip_serializing_if = "Option::is_none", rename = "quality")]
	pub quality: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClipRectangle {
	BoxClipRectangle(BoxClipRectangle),
	ElementClipRectangle(ElementClipRectangle),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ElementClipRectangle {
	#[serde(rename = "type")]
	pub r#type: ElementClipRectangleType,
	#[serde(rename = "element")]
	pub element: SharedReference,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoxClipRectangle {
	#[serde(rename = "type")]
	pub r#type: BoxClipRectangleType,
	#[serde(rename = "x")]
	pub x: f64,
	#[serde(rename = "y")]
	pub y: f64,
	#[serde(rename = "width")]
	pub width: f64,
	#[serde(rename = "height")]
	pub height: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureScreenshotResult {
    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTreeResult {
    #[serde(rename = "contexts")]
    pub contexts: InfoList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocateNodesResult {
    #[serde(rename = "nodes")]
    pub nodes: Vec<NodeRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigateResult {
    #[serde(rename = "navigation")]
    pub navigation: Option<Navigation>,
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintResult {
    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraverseHistoryResult {
    // Empty struct as per schema
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
	#[serde(rename = "method")]
	pub method: CloseMethod,
	#[serde(rename = "params")]
	pub params: CloseParameters,
}

fn close_parameters_default_prompt_unload() -> bool {
	false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseParameters {
	#[serde(rename = "promptUnload", default = "close_parameters_default_prompt_unload")]
	pub prompt_unload: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Create {
	#[serde(rename = "method")]
	pub method: CreateMethod,
	#[serde(rename = "params")]
	pub params: CreateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CreateType {
	Tab,
	Window,
}

fn create_parameters_default_background() -> bool {
	false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateParameters {
	#[serde(rename = "type")]
	pub r#type: CreateType,
	#[serde(skip_serializing_if = "Option::is_none", rename = "referenceContext")]
	pub reference_context: Option<BrowsingContext>,
	#[serde(rename = "background", default = "create_parameters_default_background")]
	pub background: bool,
	#[serde(skip_serializing_if = "Option::is_none", rename = "userContext")]
	pub user_context: Option<UserContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResult {
    #[serde(rename = "context")]
    pub context: BrowsingContext,  // BrowsingContext is already defined as String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTree {
	#[serde(rename = "method")]
	pub method: GetTreeMethod,
	#[serde(rename = "params")]
	pub params: GetTreeParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GetTreeParameters {
	#[serde(skip_serializing_if = "Option::is_none", rename = "maxDepth")]
	pub max_depth: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "root")]
	pub root: Option<BrowsingContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandleUserPrompt {
	#[serde(rename = "method")]
	pub method: HandleUserPromptMethod,
	#[serde(rename = "params")]
	pub params: HandleUserPromptParameters,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct HandleUserPromptParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(skip_serializing_if = "Option::is_none", rename = "accept")]
	pub accept: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "userText")]
	pub user_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocateNodes {
	#[serde(rename = "method")]
	pub method: LocateNodesMethod,
	#[serde(rename = "params")]
	pub params: LocateNodesParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocateNodesParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "locator")]
	pub locator: Locator,
	#[serde(skip_serializing_if = "Option::is_none", rename = "maxNodeCount")]
	pub max_node_count: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "serializationOptions")]
	pub serialization_options: Option<SerializationOptions>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "startNodes")]
	pub start_nodes: Option<Vec<SharedReference>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Navigate {
	#[serde(rename = "method")]
	pub method: NavigateMethod,
	#[serde(rename = "params")]
	pub params: NavigateParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NavigateParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "url")]
	pub url: String,
	#[serde(skip_serializing_if = "Option::is_none", rename = "wait")]
	pub wait: Option<ReadinessState>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Print {
	#[serde(rename = "method")]
	pub method: PrintMethod,
	#[serde(rename = "params")]
	pub params: PrintParameters,
}

fn print_parameters_default_background() -> bool {
	false
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PrintParametersOrientation {
	Portrait,
	Landscape,
}

fn print_parameters_default_orientation() -> PrintParametersOrientation {
	PrintParametersOrientation::Portrait
}

fn print_parameters_default_scale() -> f64 {
	1.0
}

fn print_parameters_default_shrink_to_fit() -> bool {
	true
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrintParametersPageRange {
	Number(u32),  // js-uint maps to u32 in Rust
	Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "background", default = "print_parameters_default_background")]
	pub background: bool,
	#[serde(skip_serializing_if = "Option::is_none", rename = "margin")]
	pub margin: Option<PrintMarginParameters>,
	#[serde(rename="orientation", default = "print_parameters_default_orientation")]
	pub orientation: PrintParametersOrientation,
	#[serde(skip_serializing_if = "Option::is_none", rename = "page")]
	pub page: Option<PrintPageParameters>,
	#[serde(rename = "pageRanges")]
	pub page_ranges: Vec<PrintParametersPageRange>,
	#[serde(rename = "scale", default = "print_parameters_default_scale")]
	pub scale: f64,
	#[serde(rename = "shrinkToFit", default = "print_parameters_default_shrink_to_fit")]
	pub shrink_to_fit: bool,
}

fn print_margin_parameters_default_bottom() -> f64 {
    1.0
}

fn print_margin_parameters_default_left() -> f64 {
    1.0
}

fn print_margin_parameters_default_right() -> f64 {
    1.0
}

fn print_margin_parameters_default_top() -> f64 {
    1.0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintMarginParameters {
    #[serde(rename = "bottom", default = "print_margin_parameters_default_bottom")]
    pub bottom: f64,

    #[serde(rename = "left", default = "print_margin_parameters_default_left")]
    pub left: f64,

    #[serde(rename = "right", default = "print_margin_parameters_default_right")]
    pub right: f64,

    #[serde(rename = "top", default = "print_margin_parameters_default_top")]
    pub top: f64,
}

fn print_page_parameters_default_height() -> f64 {
    27.94
}

fn print_page_parameters_default_width() -> f64 {
    21.59
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintPageParameters {
    #[serde(rename = "height", default = "print_page_parameters_default_height")]
    pub height: f64,

    #[serde(rename = "width", default = "print_page_parameters_default_width")]
    pub width: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reload {
	#[serde(rename = "method")]
	pub method: ReloadMethod,
	#[serde(rename = "params")]
	pub params: ReloadParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReloadParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(skip_serializing_if = "Option::is_none", rename = "ignoreCache")]
	pub ignore_cache: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "wait")]
	pub wait: Option<ReadinessState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetViewport {
	#[serde(rename = "method")]
	pub method: SetViewportMethod,
	#[serde(rename = "params")]
	pub params: SetViewportParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetViewportParameters {
    #[serde(skip_serializing_if = "Option::is_none", rename = "context")]
    pub context: Option<BrowsingContext>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "viewport")]
    pub viewport: Option<Viewport>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "devicePixelRatio")]
    pub device_pixel_ratio: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none", rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,  // [+] means non-empty array, but this would need runtime validation
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Viewport {
	#[serde(rename = "width")]
	pub width: u32,
	#[serde(rename = "height")]
	pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraverseHistory {
	#[serde(rename = "method")]
	pub method: TraverseHistoryMethod,
	#[serde(rename = "params")]
	pub params: TraverseHistoryParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraverseHistoryParameters {
	#[serde(rename = "context")]
	pub context: BrowsingContext,
	#[serde(rename = "delta")]
	pub delta: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReadinessState {
    None,
    Interactive,
    Complete,
}

