use winsafe_input_manager::{listener, WindowsResult};

fn main() -> WindowsResult {
	listener::init();
	for event in listener::get_event_receiver() {
		println!("Event: {:?}", event.into_event().unwrap());
	}
	Ok(())
}