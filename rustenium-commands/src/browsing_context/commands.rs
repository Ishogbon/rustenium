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
pub struct Activate {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ActivateParameters,
}

pub struct ActivateParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureScreenshot {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: CaptureScreenshotParameters,
}

pub struct CaptureScreenshotParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "origin")]
	origin: Option<String | String>,
	#[serde(rename = "format")]
	format: Option<ImageFormat>,
	#[serde(rename = "clip")]
	clip: Option<ClipRectangle>,
}

pub struct ImageFormat {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "quality")]
	quality: Option<>,
}

pub enum ClipRectangle {
	BoxClipRectangle(BoxClipRectangle),
	ElementClipRectangle(ElementClipRectangle),
}

pub struct ElementClipRectangle {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "element")]
	element: SharedReference,
}

pub struct BoxClipRectangle {
	#[serde(rename = "type")]
	type_: String,
	#[serde(rename = "x")]
	x: f64,
	#[serde(rename = "y")]
	y: f64,
	#[serde(rename = "width")]
	width: f64,
	#[serde(rename = "height")]
	height: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Close {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: CloseParameters,
}

pub struct CloseParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "promptUnload")]
	prompt_unload: Option<>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Create {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: CreateParameters,
}

pub struct CreateParameters {
	#[serde(rename = "type")]
	type_: CreateType,
	#[serde(rename = "referenceContext")]
	reference_context: Option<BrowsingContext>,
	#[serde(rename = "background")]
	background: Option<>,
	#[serde(rename = "userContext")]
	user_context: Option<UserContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTree {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: GetTreeParameters,
}

pub struct GetTreeParameters {
	#[serde(rename = "maxDepth")]
	max_depth: Option<u32>,
	#[serde(rename = "root")]
	root: Option<BrowsingContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandleUserPrompt {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: HandleUserPromptParameters,
}

pub struct HandleUserPromptParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "accept")]
	accept: Option<bool>,
	#[serde(rename = "userText")]
	user_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocateNodes {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: LocateNodesParameters,
}

pub struct LocateNodesParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "locator")]
	locator: Locator,
	#[serde(rename = "maxNodeCount")]
	max_node_count: Option<>,
	#[serde(rename = "serializationOptions")]
	serialization_options: Option<SerializationOptions>,
	#[serde(rename = "startNodes")]
	start_nodes: Option<>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Navigate {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: NavigateParameters,
}

pub struct NavigateParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "url")]
	url: String,
	#[serde(rename = "wait")]
	wait: Option<ReadinessState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Print {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: PrintParameters,
}

pub struct PrintParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "background")]
	background: Option<>,
	#[serde(rename = "margin")]
	margin: Option<PrintMarginParameters>,
	#[serde(rename = "orientation")]
	orientation: Option<String | String>,
	#[serde(rename = "page")]
	page: Option<PrintPageParameters>,
	#[serde(rename = "pageRanges")]
	page_ranges: Option< | >,
	#[serde(rename = "scale")]
	scale: Option<>,
	#[serde(rename = "shrinkToFit")]
	shrink_to_fit: Option<>,
}

pub struct PrintMarginParameters {
	#[serde(rename = "bottom")]
	bottom: Option<>,
	#[serde(rename = "left")]
	left: Option<>,
	#[serde(rename = "right")]
	right: Option<>,
	#[serde(rename = "top")]
	top: Option<>,
}

pub struct PrintPageParameters {
	#[serde(rename = "height")]
	height: Option<>,
	#[serde(rename = "width")]
	width: Option<>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Reload {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: ReloadParameters,
}

pub struct ReloadParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "ignoreCache")]
	ignore_cache: Option<bool>,
	#[serde(rename = "wait")]
	wait: Option<ReadinessState>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetViewport {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: SetViewportParameters,
}

pub struct SetViewportParameters {
	#[serde(rename = "context")]
	context: Option<BrowsingContext>,
	#[serde(rename = "viewport")]
	viewport: Option<Viewport | None>,
	#[serde(rename = "devicePixelRatio")]
	device_pixel_ratio: Option< | None>,
	#[serde(rename = "userContexts")]
	user_contexts: Option<Vec<UserContext>>,
}

pub struct Viewport {
	#[serde(rename = "width")]
	width: u32,
	#[serde(rename = "height")]
	height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TraverseHistory {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: TraverseHistoryParameters,
}

pub struct TraverseHistoryParameters {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "delta")]
	delta: i32,
}

