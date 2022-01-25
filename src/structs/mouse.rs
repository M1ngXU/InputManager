use std::fmt::Debug;
use winapi::um::winuser::*;
use winapi::shared::minwindef::DWORD;
use crate::sender::{Event, EventQueue};
use crate::util::{i32_to_u32};

/// Different mouse events:
///
/// `SetCursor(x, y)`: Sets the cursor position to x|y (0|0 = top|left), negative with multiple monitors
///
/// `MoveCursor(dx, dy)`: Moves the cursor by dx|dy (position shouldn't exceed -32k/32k) (0|0 = top|left)
///
/// `ButtonDown(b)`: Presses the Button b
///
/// `ButtonUp(b)`: Releases the button b
///
/// `ButtonDoubleClick(b)`: Listen-only event weather b was double clicked, requires [CS_DBLCLKS](https://docs.microsoft.com/en-us/windows/win32/inputdev/wm-lbuttondblclk?redirectedfrom=MSDN#:~:text=Only%20windows%20that%20have%20the%20CS_DBLCLKS%20style%20can%20receive%20WM_LBUTTONDBLCLK%20messages%2C%20which%20the%20system%20generates%20whenever%20the%20user%20presses%2C%20releases%2C%20and%20again%20presses%20the%20left%20mouse%20button%20within%20the%20system%27s%20double%2Dclick%20time%20limit).
///
/// `ScrollHorizontal(a)`: Scrolls `a` `MOUSE_DELTAS` horizontally. (positive = scroll right)
///
/// `ScrollVertical(a)`: Scrolls `a` `MOUSE_DELTAS` vertically. (positive = scroll up)
#[derive(Debug)]
pub enum MouseEvent {
	SetCursor(i16, i16),
	MoveCursor(i32, i32),
	ButtonDown(MouseButton),
	ButtonUp(MouseButton),
	// listen only
	ButtonDoubleClick(MouseButton),
	ScrollHorizontal(i32),
	ScrollVertical(i32)
}
impl Event for MouseEvent {
	fn into_event_queue(self) -> EventQueue {
		match self {
			MouseEvent::SetCursor(x, y) => _move(x as i32, y as i32, false),
			MouseEvent::MoveCursor(x, y) => _move(x, y, true),
			MouseEvent::ButtonDown(b) => _button(b, false),
			MouseEvent::ButtonUp(b) => _button(b, true),
			MouseEvent::ScrollHorizontal(a) => _scroll(a, MOUSEEVENTF_HWHEEL),
			MouseEvent::ScrollVertical(a) => _scroll(a, MOUSEEVENTF_WHEEL),
			e => panic!("Cannot convert {:?} to EventQueue.", e)
		}.into()
	}
}

#[derive(Debug)]
pub enum MouseButton {
	Left,
	Middle,
	Right
}
impl MouseButton {
	fn get_down_flag(&self) -> DWORD {
		match self {
			MouseButton::Left => MOUSEEVENTF_LEFTDOWN,
			MouseButton::Right => MOUSEEVENTF_RIGHTDOWN,
			MouseButton::Middle => MOUSEEVENTF_MIDDLEDOWN
		}
	}

	fn get_up_flag(&self) -> DWORD {
		match self {
			MouseButton::Left => MOUSEEVENTF_LEFTUP,
			MouseButton::Right => MOUSEEVENTF_RIGHTUP,
			MouseButton::Middle => MOUSEEVENTF_MIDDLEUP
		}
	}
}

fn _scroll(amount: i32, flags: DWORD) -> INPUT {
	_get_by_data_and_flag(i32_to_u32(amount * WHEEL_DELTA as i32), flags)
}

fn _get_by_data_and_flag(data: u32, flags: DWORD) -> INPUT {
	get_mouse_input(0, 0, data, flags)
}

fn _button(b: MouseButton, up: bool) -> INPUT {
	_get_by_data_and_flag(0, if up { b.get_up_flag() } else { b.get_down_flag() })
}

fn _move(x: i32, y: i32, rel: bool) -> INPUT {
	get_mouse_input(x, y, 0, MOUSEEVENTF_MOVE | if rel { 0 } else { MOUSEEVENTF_ABSOLUTE })
}

pub fn get_mouse_input(dx: i32, dy: i32, mouse_data: DWORD, dw_flags: DWORD) -> INPUT {
	let mut union: INPUT_u = unsafe { std::mem::zeroed() };
	let inner_union = unsafe { union.mi_mut() };
	*inner_union = MOUSEINPUT {
		dx,
		dy,
		mouseData: mouse_data,
		dwFlags: dw_flags,
		time: 0,
		dwExtraInfo: 0
	};
	INPUT {
		type_: INPUT_MOUSE,
		u: union,
	}
}