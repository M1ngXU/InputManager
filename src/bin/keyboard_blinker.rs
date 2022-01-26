use winsafe_input_manager::{queue, WindowsResult};
use winsafe_input_manager::structs::keyboard::{VirtualKey,KeyboardEvent};

fn main() -> WindowsResult {
	let mut i = 2;
	let mut p = 3;
	let vk = [ VirtualKey::NumLock, VirtualKey::CapsLock, VirtualKey::Scroll ];
	loop {
		i = (i + 1) % 3;
		queue!(KeyboardEvent::VirtualKeyDown(vk[i]), KeyboardEvent::VirtualKeyUp(vk[i]));
		if p < 3 {
			queue!(KeyboardEvent::VirtualKeyDown(vk[p]), KeyboardEvent::VirtualKeyUp(vk[p]));
		}
		p = i;
		std::thread::sleep(std::time::Duration::from_millis(250));
	}
}