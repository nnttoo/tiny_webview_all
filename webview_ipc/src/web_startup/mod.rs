mod icon;
mod response;
mod response_command;
mod response_uiapi;
mod response_uiapi_commmand;
mod startup;
mod web;

use std::error::Error;

pub use startup::start_by_json;
pub type BoxError = Box<dyn Error>;