use std::io::Error;

pub type WindowsResult = Result<(), Error>;

#[macro_use]
pub mod sender;
mod util;
pub mod listener;
pub mod structs;
