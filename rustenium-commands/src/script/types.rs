use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::browsing_context::types::BrowsingContext;

pub type Channel = String;
pub type Realm = String;
pub type ListLocalValue = Vec<LocalValue>;
pub type SharedId = String;
pub type Handle = String;
pub type InternalId = String;
pub type ListRemoteValue = Vec<RemoteValue>;
pub type PreloadScript = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum SpecialNumber {
	#[serde(rename = "NaN")]
	NaN,

	#[serde(rename = "-0")]
	NegativeZero,

	#[serde(rename = "Infinity")]
	Infinity,

	#[serde(rename = "-Infinity")]
	NegativeInfinity,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Number {
	Int(i64),
	Float(f64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LocalValueOrText {
	LocalValue(LocalValue),
	Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingLocalValueEntry(
	pub LocalValueOrText,
	pub LocalValue,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingLocalValue(
	pub Vec<MappingLocalValueEntry>,
);


#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoteValueOrText {
	RemoteValue(RemoteValue),
	Text(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingRemoteValueEntry(
	pub RemoteValueOrText,
	pub RemoteValue,
);

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingRemoteValue(
	pub Vec<MappingRemoteValueEntry>
);


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResultOwnership {
	Root,
	None,
}

#[derive(Debug, Serialize, Deserialize)]
enum ChannelValueType {
	#[serde(rename = "channel")]
	Channel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelValue {
	#[serde(rename = "type")]
	r#type: ChannelValueType,
	#[serde(rename = "value")]
	value: ChannelProperties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChannelProperties {
	#[serde(rename = "channel")]
	channel: Channel,
	#[serde(rename = "serializationOptions")]
	serialization_options: Option<SerializationOptions>,
	#[serde(rename = "ownership")]
	ownership: Option<ResultOwnership>,
}

pub enum EvaluateResult {
	EvaluateResultSuccess(EvaluateResultSuccess),
	EvaluateResultException(EvaluateResultException),
}

#[derive(Debug, Serialize, Deserialize)]
enum EvaluateResultSuccessType {
	#[serde(rename = "success")]
	Success,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultSuccess {
	#[serde(rename = "type")]
	r#type: EvaluateResultSuccessType,
	#[serde(rename = "result")]
	result: RemoteValue,
	#[serde(rename = "realm")]
	realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
enum EvaluateResultExceptionType {
	#[serde(rename = "exception")]
	Exception,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluateResultException {
	#[serde(rename = "type")]
	r#type: EvaluateResultExceptionType,
	#[serde(rename = "exceptionDetails")]
	exception_details: ExceptionDetails,
	#[serde(rename = "realm")]
	realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExceptionDetails {
	#[serde(rename = "columnNumber")]
	column_number: u32,
	#[serde(rename = "exception")]
	exception: RemoteValue,
	#[serde(rename = "linpub enumber")]
	line_number: u32,
	#[serde(rename = "stackTrace")]
	stack_trace: StackTrace,
	#[serde(rename = "text")]
	text: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum LocalValue {
	RemoteReference(RemoteReference),
	PrimitiveProtocolValue(PrimitiveProtocolValue),
	ChannelValue(ChannelValue),
	ArrayLocalValue(ArrayLocalValue),
	DateLocalValue(DateLocalValue),
	MapLocalValue(MapLocalValue),
	ObjectLocalValue(ObjectLocalValue),
	RegExpLocalValue(RegExpLocalValue),
	SetLocalValue(SetLocalValue),
}

#[derive(Debug, Serialize, Deserialize)]
enum ArrayLocalValueType {
	#[serde(rename = "array")]
	Array,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayLocalValue {
	#[serde(rename = "type")]
	r#type: ArrayLocalValueType,
	#[serde(rename = "value")]
	value: ListLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum DateLocalValueType {
	#[serde(rename = "date")]
	Date,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateLocalValue {
	#[serde(rename = "type")]
	r#type: DateLocalValueType,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum MapLocalValueType {
	#[serde(rename = "map")]
	Map,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapLocalValue {
	#[serde(rename = "type")]
	r#type: MapLocalValueType,
	#[serde(rename = "value")]
	value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum ObjectLocalValueType {
	#[serde(rename = "object")]
	Object,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectLocalValue {
	#[serde(rename = "type")]
	r#type: ObjectLocalValueType,
	#[serde(rename = "value")]
	value: MappingLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpValue {
	#[serde(rename = "pattern")]
	pattern: String,
	#[serde(rename = "flags")]
	flags: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum RegExpLocalValueType {
	#[serde(rename = "regexp")]
	RegExp,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpLocalValue {
	#[serde(rename = "type")]
	r#type: RegExpLocalValueType,
	#[serde(rename = "value")]
	value: RegExpValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum SetLocalValueType {
	#[serde(rename = "set")]
	Set,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetLocalValue {
	#[serde(rename = "type")]
	r#type: SetLocalValueType,
	#[serde(rename = "value")]
	value: ListLocalValue,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PrimitiveProtocolValue {
	UndefinedValue(UndefinedValue),
	NullValue(NullValue),
	StringValue(StringValue),
	NumberValue(NumberValue),
	BooleanValue(BooleanValue),
	BigIntValue(BigIntValue),
}

#[derive(Debug, Serialize, Deserialize)]
enum UndefinedValueType {
	#[serde(rename = "undefined")]
	Undefined,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UndefinedValue {
	#[serde(rename = "type")]
	r#type: UndefinedValueType,
}

#[derive(Debug, Serialize, Deserialize)]
enum NullValueType {
	#[serde(rename = "null")]
	Null,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NullValue {
	#[serde(rename = "type")]
	r#type: NullValueType,
}

#[derive(Debug, Serialize, Deserialize)]
enum StringValueType {
	#[serde(rename = "string")]
	String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StringValue {
	#[serde(rename = "type")]
	r#type: StringValueType,
	#[serde(rename = "value")]
	value: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum NumberValueType {
	#[serde(rename = "number")]
	Number,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberValueValue {
	Number(Number),
	SpecialNumber(SpecialNumber),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NumberValue {
	#[serde(rename = "type")]
	r#type: NumberValueType,
	#[serde(rename = "value")]
	value: NumberValueValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum BooleanValueType {
	#[serde(rename = "boolean")]
	Boolean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BooleanValue {
	#[serde(rename = "type")]
	r#type: BooleanValueType,
	#[serde(rename = "value")]
	value: bool,
}

#[derive(Debug, Serialize, Deserialize)]
enum BigIntValueType {
	#[serde(rename = "bigint")]
	BigInt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BigIntValue {
	#[serde(rename = "type")]
	r#type: BigIntValueType,
	#[serde(rename = "value")]
	value: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub enum RealmInfo {
	WindowRealmInfo(WindowRealmInfo),
	DedicatedWorkerRealmInfo(DedicatedWorkerRealmInfo),
	SharedWorkerRealmInfo(SharedWorkerRealmInfo),
	ServiceWorkerRealmInfo(ServiceWorkerRealmInfo),
	WorkerRealmInfo(WorkerRealmInfo),
	PaintWorkletRealmInfo(PaintWorkletRealmInfo),
	AudioWorkletRealmInfo(AudioWorkletRealmInfo),
	WorkletRealmInfo(WorkletRealmInfo),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BaseRealmInfo {
	#[serde(rename = "realm")]
	realm: Realm,
	#[serde(rename = "origin")]
	origin: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RealmType {
	Window,
	DedicatedWorker,
	SharedWorker,
	ServiceWorker,
	Worker,
	PaintWorklet,
	AudioWorklet,
	Worklet,
}

#[derive(Debug, Serialize, Deserialize)]
enum WindowRealmInfoType {
	#[serde(rename = "window")]
	Window,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowRealmInfo {
	#[serde(flatten)]
base: BaseRealmInfo,
	#[serde(rename = "type")]
r#type: WindowRealmInfoType,
	#[serde(rename = "context")]
context: BrowsingContext,
	#[serde(rename = "sandbox")]
sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum DedicatedWorkerRealmInfoType {
	#[serde(rename = "dedicated-worker")]
	DedicatedWorker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DedicatedWorkerRealmInfo {
	#[serde(flatten)]
base: BaseRealmInfo,
	#[serde(rename="type")]
r#type: DedicatedWorkerRealmInfoType,
	#[serde(rename = "owners")]
owners: Vec<Realm>
}

#[derive(Debug, Serialize, Deserialize)]
enum SharedWorkerRealmInfoType {
	#[serde(rename = "shared-worker")]
	SharedWorker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedWorkerRealmInfo {
	#[serde(flatten)]
base: BaseRealmInfo,
#[serde(rename="type")]
r#type: SharedWorkerRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum ServiceWorkerRealmInfoType {
	#[serde(rename = "service-worker")]
	ServiceWorker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceWorkerRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: ServiceWorkerRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum WorkerRealmInfoType {
	#[serde(rename = "worker")]
	Worker,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkerRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: WorkerRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum PaintWorkletRealmInfoType {
	#[serde(rename = "paint-worklet")]
	PaintWorklet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaintWorkletRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: PaintWorkletRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum AudioWorkletRealmInfoType {
	#[serde(rename = "audio-worklet")]
	AudioWorklet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AudioWorkletRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: AudioWorkletRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
enum WorkletRealmInfoType {
	#[serde(rename = "worklet")]
	Worklet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkletRealmInfo {
	#[serde(flatten)]
	base: BaseRealmInfo,
	#[serde(rename = "type")]
	r#type: WorkletRealmInfoType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoteReference {
	SharedReference(SharedReference),
	RemoteObjectReference(RemoteObjectReference),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SharedReference {
	#[serde(rename = "sharedId")]
	shared_id: SharedId,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RemoteObjectReference {
	#[serde(rename = "handle")]
	handle: Handle,
	#[serde(rename = "sharedId")]
	shared_id: Option<SharedId>,
	#[serde(flatten)]
	extension: Option<serde_cbor::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RemoteValue {
	PrimitiveProtocolValue(PrimitiveProtocolValue),
	SymbolRemoteValue(SymbolRemoteValue),
	ArrayRemoteValue(ArrayRemoteValue),
	ObjectRemoteValue(ObjectRemoteValue),
	FunctionRemoteValue(FunctionRemoteValue),
	RegExpRemoteValue(RegExpRemoteValue),
	DateRemoteValue(DateRemoteValue),
	MapRemoteValue(MapRemoteValue),
	SetRemoteValue(SetRemoteValue),
	WeakMapRemoteValue(WeakMapRemoteValue),
	WeakSetRemoteValue(WeakSetRemoteValue),
	GeneratorRemoteValue(GeneratorRemoteValue),
	ErrorRemoteValue(ErrorRemoteValue),
	ProxyRemoteValue(ProxyRemoteValue),
	PromiseRemoteValue(PromiseRemoteValue),
	TypedArrayRemoteValue(TypedArrayRemoteValue),
	ArrayBufferRemoteValue(ArrayBufferRemoteValue),
	NodeListRemoteValue(NodeListRemoteValue),
	HTMLCollectionRemoteValue(HTMLCollectionRemoteValue),
	NodeRemoteValue(NodeRemoteValue),
	WindowProxyRemoteValue(WindowProxyRemoteValue),
}

#[derive(Debug, Serialize, Deserialize)]
enum SymbolRemoteValueType {
	#[serde(rename = "symbol")]
	Symbol,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbolRemoteValue {
	#[serde(rename = "type")]
	r#type: SymbolRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ArrayRemoteValueType {
	#[serde(rename = "array")]
	Array,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayRemoteValue {
	#[serde(rename = "type")]
	r#type: ArrayRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ObjectRemoteValueType {
	#[serde(rename = "object")]
	Object,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectRemoteValue {
	#[serde(rename = "type")]
	r#type: ObjectRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<MappingRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum FunctionRemoteValueType {
	#[serde(rename = "function")]
	Function,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunctionRemoteValue {
	#[serde(rename = "type")]
	r#type: FunctionRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegExpRemoteValue {
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,

	#[serde(flatten)]
	local: RegExpLocalValue,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DateRemoteValue {
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(flatten)]
	local: DateLocalValue,
}

#[derive(Debug, Serialize, Deserialize)]
enum MapRemoteValueType {
	#[serde(rename = "map")]
	Map,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapRemoteValue {
	#[serde(rename = "type")]
	r#type: MapRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<MappingRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum SetRemoteValueType {
	#[serde(rename = "set")]
	Set,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetRemoteValue {
	#[serde(rename = "type")]
	r#type: SetRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum WeakMapRemoteValueType {
	#[serde(rename = "weakmap")]
	WeakMap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakMapRemoteValue {
	#[serde(rename = "type")]
	r#type: WeakMapRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum WeakSetRemoteValueType {
	#[serde(rename = "weakset")]
	WeakSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeakSetRemoteValue {
	#[serde(rename = "type")]
	r#type: WeakSetRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum GeneratorRemoteValueType {
	#[serde(rename = "generator")]
	Generator,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratorRemoteValue {
	#[serde(rename = "type")]
	r#type: GeneratorRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ErrorRemoteValueType {
	#[serde(rename = "error")]
	Error,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorRemoteValue {
	#[serde(rename = "type")]
	r#type: ErrorRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ProxyRemoteValueType {
	#[serde(rename = "proxy")]
	Proxy,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyRemoteValue {
	#[serde(rename = "type")]
	r#type: ProxyRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum PromiseRemoteValueType {
	#[serde(rename = "promise")]
	Promise,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromiseRemoteValue {
	#[serde(rename = "type")]
	r#type: PromiseRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum TypedArrayRemoteValueType {
	#[serde(rename = "typedarray")]
	TypedArray,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypedArrayRemoteValue {
	#[serde(rename = "type")]
	r#type: TypedArrayRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum ArrayBufferRemoteValueType {
	#[serde(rename = "arraybuffer")]
	ArrayBuffer,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArrayBufferRemoteValue {
	#[serde(rename = "type")]
	r#type: ArrayBufferRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
enum NodeListRemoteValueType {
	#[serde(rename = "nodelist")]
	NodeList,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeListRemoteValue {
	#[serde(rename = "type")]
	r#type: NodeListRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum HTMLCollectionRemoteValueType {
	#[serde(rename = "htmlcollection")]
	HtmlCollection,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HTMLCollectionRemoteValue {
	#[serde(rename = "type")]
	r#type: HTMLCollectionRemoteValueType,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<ListRemoteValue>,
}

#[derive(Debug, Serialize, Deserialize)]
enum NodeRemoteValueType {
	#[serde(rename = "node")]
	Node,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeRemoteValue {
	#[serde(rename = "type")]
	r#type: NodeRemoteValueType,
	#[serde(rename = "sharedId")]
	shared_id: Option<SharedId>,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
	#[serde(rename = "value")]
	value: Option<Box<NodeProperties>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodePropertiesMode {
	Open,
	Closed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeProperties {
	#[serde(rename = "nodeType")]
	node_type: u32,
	#[serde(rename = "childNodeCount")]
	child_node_count: u32,
	#[serde(rename = "attributes")]
	attributes: Option<HashMap<String, String>>,
	#[serde(rename = "children")]
	children: Option<Vec<NodeRemoteValue>>,
	#[serde(rename = "localName")]
	local_name: Option<String>,
	#[serde(rename = "mode")]
	mode: Option<NodePropertiesMode>,
	#[serde(rename = "namespaceURI")]
	namespace_u_r_i: Option<String>,
	#[serde(rename = "nodeValue")]
	node_value: Option<String>,
	#[serde(rename = "shadowRoot")]
	shadow_root: Option<Box<NodeRemoteValue>>,
}

#[derive(Debug, Serialize, Deserialize)]
enum WindowProxyRemoteValueType {
	#[serde(rename = "window")]
	Window,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowProxyRemoteValue {
	#[serde(rename = "type")]
	r#type: WindowProxyRemoteValueType,
	#[serde(rename = "value")]
	value: WindowProxyProperties,
	#[serde(rename = "handle")]
	handle: Option<Handle>,
	#[serde(rename = "internalId")]
	internal_id: Option<InternalId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowProxyProperties {
	#[serde(rename = "context")]
	context: BrowsingContext,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IncludeShadowTree {
	None,
	Open,
	All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SerializationOptions {
	#[serde(rename = "maxDomDepth", default = "serialization_options_default_max_dom_depth")]
	pub max_dom_depth: Option<u32>,
	
	#[serde(rename = "maxObjectDepth", default)]
	pub max_object_depth: Option<u32>,
	
	#[serde(rename = "includeShadowTree", default = "serialization_options_default_include_shadow_tree")]
	pub include_shadow_tree: IncludeShadowTree,
}

fn serialization_options_default_max_dom_depth() -> Option<u32> {
	Some(0)
}

fn serialization_options_default_include_shadow_tree() -> IncludeShadowTree {
	IncludeShadowTree::None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackFrame {
	#[serde(rename = "columnNumber")]
	column_number: u32,
	#[serde(rename = "functionName")]
	function_name: String,
	#[serde(rename = "linpub enumber")]
	line_number: u32,
	#[serde(rename = "url")]
	url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackTrace {
	#[serde(rename = "callFrames")]
	call_frames: Vec<StackFrame>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
	#[serde(rename = "realm")]
	realm: Realm,
	#[serde(rename = "context")]
	context: Option<BrowsingContext>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RealmTarget {
	#[serde(rename = "realm")]
	realm: Realm,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextTarget {
	#[serde(rename = "context")]
	context: BrowsingContext,
	#[serde(rename = "sandbox")]
	sandbox: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Target {
	ContextTarget(ContextTarget),
	RealmTarget(RealmTarget),
}