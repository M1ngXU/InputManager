use input_manager::{queue, WindowsResult};
use input_manager::structs::keyboard::{VirtualKey, KeyboardEvent};

fn main() -> WindowsResult {
	let mut i = 0;
	let vk = [ VirtualKey::NumLock, VirtualKey::CapsLock, VirtualKey::Scroll ];
	loop {
		i = (i + 1) % 3;
		queue!(KeyboardEvent::VirtualKeyDown(vk[i]), KeyboardEvent::VirtualKeyUp(vk[i]));
		std::thread::sleep(std::time::Duration::from_millis(5));
	}
}