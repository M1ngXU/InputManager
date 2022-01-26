use winsafe_input_manager::structs::mouse::{MouseEvent,MouseButton};
use winsafe_input_manager::{queue, WindowsResult};
use winsafe_input_manager::sender::Event;

fn main() -> WindowsResult {
	MouseEvent::ScrollVertical(-10).into_event_queue().execute()?;
	MouseEvent::ScrollHorizontal(-10).into_event_queue().execute()?;
	queue!(
		MouseEvent::SetCursor(0, 0),
		MouseEvent::MoveCursor(50, 50)
	);
	std::thread::sleep(std::time::Duration::from_secs(1));
	queue!(
		MouseEvent::ButtonDown(MouseButton::Left),
		MouseEvent::ButtonUp(MouseButton::Left),
		MouseEvent::ButtonDown(MouseButton::Middle),
		MouseEvent::ButtonUp(MouseButton::Middle),
		MouseEvent::ButtonDown(MouseButton::Right),
		MouseEvent::ButtonUp(MouseButton::Right)
	);
	Ok(())
}