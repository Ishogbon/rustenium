use serde::{Serialize, Deserialize};

use crate::{browser::types::UserContext, browsing_context::types::BrowsingContext};

#[derive(Debug, Serialize, Deserialize)]
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
	#[serde(rename = "altitude", default = "geolocation_coordinates_default_altitude")]
	pub altitude: Option<f64>,
	#[serde(rename = "altitudeAccuracy", default = "geolocation_coordinates_default_altitude_accuracy")]
	pub altitude_accuracy: Option<f64>,
	#[serde(rename = "heading", default = "geolocation_coordinates_default_heading")]
	pub heading: Option<f64>,
	#[serde(rename = "speed", default = "geolocation_coordinates_default_speed")]
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
	#[serde(rename = "contexts")]
	pub contexts: Option<Vec<BrowsingContext>>,
	#[serde(rename = "userContexts")]
	pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetGeolocationOverride {
	#[serde(rename = "method")]
	pub method: SetGeolocationOverrideMethod,
	#[serde(rename = "params")]
	pub params: SetGeolocationOverrideParameters,
}

