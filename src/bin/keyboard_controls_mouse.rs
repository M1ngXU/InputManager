use winsafe_input_manager::{listener, WindowsResult};
use winsafe_input_manager::listener::EventType;
use winsafe_input_manager::sender::Event;
use winsafe_input_manager::structs::keyboard::{KeyboardEvent, VirtualKey};
use winsafe_input_manager::structs::mouse::{MouseButton, MouseEvent};

fn main() -> WindowsResult {
	listener::init();
	let recv = listener::get_event_receiver();

	let movement_speed = 1000.0;
	let delta_time = 0.01;
	let shift_multiplier = 5.0;

	let mut left = false;
	let mut up = false;
	let mut right = false;
	let mut down = false;
	let mut shift = false;

	loop {
		while let Ok(c) = recv.try_recv() {
			if let Some(EventType::KeyboardEvent(k)) = c.into_event() {
				match k {
					KeyboardEvent::VirtualKeyDown(VirtualKey::ArrowLeft) => left = true,
					KeyboardEvent::VirtualKeyDown(VirtualKey::ArrowUp) => up = true,
					KeyboardEvent::VirtualKeyDown(VirtualKey::ArrowRight) => right = true,
					KeyboardEvent::VirtualKeyDown(VirtualKey::ArrowDown) => down = true,
					KeyboardEvent::VirtualKeyDown(VirtualKey::LeftShift) => shift = true,
					KeyboardEvent::VirtualKeyDown(VirtualKey::Enter) => MouseEvent::ButtonDown(MouseButton::Left).into_event_queue().execute()?,

					KeyboardEvent::VirtualKeyUp(VirtualKey::ArrowLeft) => left = false,
					KeyboardEvent::VirtualKeyUp(VirtualKey::ArrowUp) => up = false,
					KeyboardEvent::VirtualKeyUp(VirtualKey::ArrowRight) => right = false,
					KeyboardEvent::VirtualKeyUp(VirtualKey::ArrowDown) => down = false,
					KeyboardEvent::VirtualKeyUp(VirtualKey::LeftShift) => shift = false,
					KeyboardEvent::VirtualKeyUp(VirtualKey::Enter) => MouseEvent::ButtonUp(MouseButton::Left).into_event_queue().execute()?,

					_ => {}
				}
			}
		}

		let mut new_pos = (0.0, 0.0);
		if left {
			new_pos.0 -= movement_speed;
		}
		if up {
			new_pos.1 -= movement_speed;
		}
		if right {
			new_pos.0 += movement_speed;
		}
		if down {
			new_pos.1 += movement_speed;
		}
		if shift {
			new_pos.0 *= shift_multiplier;
			new_pos.1 *= shift_multiplier;
		}
		new_pos.0 *= delta_time;
		new_pos.1 *= delta_time;
		MouseEvent::MoveCursor(new_pos.0 as i32, new_pos.1 as i32).into_event_queue().execute()?;
		std::thread::sleep(std::time::Duration::from_millis((1_000.0 * delta_time) as u64));
	}
}