use serde::{Serialize, Deserialize};

use crate::{browser::types::UserContext, browsing_context::types::BrowsingContext};

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationCommand {
	SetGeolocationOverride(SetGeolocationOverride),
}

fn geolocation_coordinates_default_accuracy() -> f64 {
	1.0
}

fn geolocation_coordinates_default_altitude() -> Option<f64> {
	None
}

fn geolocation_coordinates_default_altitude_accuracy() -> Option<f64> {
	None
}

fn geolocation_coordinates_default_heading() -> Option<f64> {
	None
}

fn geolocation_coordinates_default_speed() -> Option<f64> {
	None
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeolocationCoordinates {
	#[serde(rename = "latitude")]
	pub latitude: f64,
	#[serde(rename = "longitude")]
	pub longitude: f64,
	#[serde(rename = "accuracy", default = "geolocation_coordinates_default_accuracy")]
	pub accuracy: f64,
	#[serde(skip_serializing_if = "Option::is_none", rename = "altitude", default = "geolocation_coordinates_default_altitude")]
	pub altitude: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "altitudeAccuracy", default = "geolocation_coordinates_default_altitude_accuracy")]
	pub altitude_accuracy: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "heading", default = "geolocation_coordinates_default_heading")]
	pub heading: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "speed", default = "geolocation_coordinates_default_speed")]
	pub speed: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SetGeolocationOverrideMethod {
	#[serde(rename = "emulation.setGeolocationOverride")]
	EmulationSetGeolocationOverride,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetGeolocationOverrideParameters {
	#[serde(rename = "coordinates")]
	pub coordinates: Option<GeolocationCoordinates>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "contexts")]
	pub contexts: Option<Vec<BrowsingContext>>,
	#[serde(skip_serializing_if = "Option::is_none", rename = "userContexts")]
	pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetGeolocationOverride {
	#[serde(rename = "method")]
	pub method: SetGeolocationOverrideMethod,
	#[serde(rename = "params")]
	pub params: SetGeolocationOverrideParameters,
}

