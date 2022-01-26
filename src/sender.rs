use std::fmt::Debug;
use std::mem::size_of;

use winapi::ctypes::c_int;
use winapi::shared::minwindef::UINT;
use winapi::um::winuser::{INPUT, SendInput};
use std::io::Error;
use crate::WindowsResult;

pub struct EventQueue {
	events: Vec<INPUT>
}
impl EventQueue {
	pub fn new() -> Self {
		Self { events: Vec::new() }
	}

	pub fn push(&mut self, q: Self) {
		for e in q.events {
			self.events.push(e);
		}
	}

	pub fn execute(&mut self) -> WindowsResult {
		unsafe {
			if SendInput(
				self.events.len() as UINT,
				self.events.as_mut_ptr(),
				size_of::<INPUT>() as c_int,
			) == self.events.len() as u32 {
				Ok(())
			} else {
				Err(Error::last_os_error())
			}
		}
	}
}
impl Default for EventQueue {
	fn default() -> Self {
		Self::new()
	}
}
impl From<INPUT> for EventQueue {
	fn from(i: INPUT) -> Self {
		EventQueue { events: vec![ i ] }
	}
}
pub trait Event: Debug {
	fn into_event_queue(self) -> EventQueue;
}
/// simulates all given events, by queuing them first and then sending them at once
#[macro_export]
macro_rules! queue {
	($($e:expr), +) => {
		let mut q = ::input_manager::sender::EventQueue::new();
		$(
			q.push(::input_manager::sender::Event::into_event_queue($e));
		)+
		q.execute()?;
	}
}