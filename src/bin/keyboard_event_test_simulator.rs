use winsafe_input_manager::WindowsResult;
use winsafe_input_manager::listener::EventType;
use winsafe_input_manager::structs::keyboard::{VirtualKey, KeyboardEvent};
use winsafe_input_manager::sender::*;

fn main() -> WindowsResult {
	KeyboardEvent::VirtualKeyDown(VirtualKey::VolumeMute).into_event_queue().execute()?;
	winsafe_input_manager::listener::init();
	for event in winsafe_input_manager::listener::get_event_receiver().into_iter() {
		if let Some(EventType::KeyboardEvent(k)) =  event.into_event() {
			println!("{:?}", k);
		}
	}
	Ok(())
}