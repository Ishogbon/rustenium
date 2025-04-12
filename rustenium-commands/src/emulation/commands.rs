#[derive(Debug, Serialize, Deserialize)]
pub struct SetGeolocationOverride {
	#[serde(rename = "method")]
	method: String,
	#[serde(rename = "params")]
	params: SetGeolocationOverrideParameters,
}

pub enum SetGeolocationOverrideParameters {
	#[serde(rename = "coordinates")]
	coordinates: GeolocationCoordinates | None,
	#[serde(rename = "contexts")]
	contexts: Option<Vec<BrowsingContext>>,
	#[serde(rename = "userContexts")]
	user_contexts: Option<Vec<UserContext>>,
}

pub struct GeolocationCoordinates {
	#[serde(rename = "latitude")]
	latitude: f64,
	#[serde(rename = "longitude")]
	longitude: f64,
	#[serde(rename = "accuracy")]
	accuracy: Option<>,
	#[serde(rename = "altitude")]
	altitude: Option<f64 | >,
	#[serde(rename = "altitudeAccuracy")]
	altitude_accuracy: Option<f64 | >,
	#[serde(rename = "heading")]
	heading: Option<f64 | >,
	#[serde(rename = "speed")]
	speed: Option<f64 | >,
}

