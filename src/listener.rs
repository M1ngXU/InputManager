use std::fmt::Debug;
use std::ptr::null_mut;
use std::sync::mpsc;
use winapi::um::winuser::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use winapi::ctypes::c_int;
use crate::structs::keyboard::{KeyboardEvent, VirtualKey};
use crate::structs::mouse::{MouseButton, MouseEvent};
use crate::util::{get_high_order, u16_as_i16};

#[derive(Debug)]
pub enum EventType {
	KeyboardEvent(KeyboardEvent),
	MouseEvent(MouseEvent)
}

#[derive(Debug)]
pub struct Callback {
	code: i32,
	param: usize,
	lp_data: isize,
}
impl Callback {
	fn new(code: i32, param: usize, lp_data: isize) -> Self {
		Self{
			code,
			param,
			lp_data
		}
	}

	pub fn into_event(self) -> Option<EventType> {
		let p = self.param as u32;
		if self.code == HC_ACTION {
			if (WM_KEYFIRST..=WM_KEYLAST).contains(&p) {
				let data = unsafe { *(self.lp_data as *const KBDLLHOOKSTRUCT) };
				Some(EventType::KeyboardEvent(match p {
					WM_KEYUP | WM_SYSKEYUP => KeyboardEvent::VirtualKeyUp(VirtualKey::try_from(data.vkCode).unwrap()),
					WM_KEYDOWN | WM_SYSKEYDOWN => KeyboardEvent::VirtualKeyDown(VirtualKey::try_from(data.vkCode).unwrap()),
					WM_CHAR | WM_SYSCHAR => KeyboardEvent::Literal(char::from_u32(data.scanCode).unwrap()),
					_ => return None
				}))
			} else if (WM_MOUSEFIRST..=WM_MOUSELAST).contains(&p) {
				Some(EventType::MouseEvent(match p {
					WM_LBUTTONUP => MouseEvent::ButtonUp(MouseButton::Left),
					WM_LBUTTONDOWN => MouseEvent::ButtonDown(MouseButton::Left),
					WM_LBUTTONDBLCLK => MouseEvent::ButtonDoubleClick(MouseButton::Left),
					WM_MBUTTONUP => MouseEvent::ButtonUp(MouseButton::Middle),
					WM_MBUTTONDOWN => MouseEvent::ButtonDown(MouseButton::Middle),
					WM_MBUTTONDBLCLK => MouseEvent::ButtonDoubleClick(MouseButton::Middle),
					WM_RBUTTONUP => MouseEvent::ButtonUp(MouseButton::Right),
					WM_RBUTTONDOWN => MouseEvent::ButtonDown(MouseButton::Right),
					WM_RBUTTONDBLCLK => MouseEvent::ButtonDoubleClick(MouseButton::Right),
					WM_MOUSEMOVE => unsafe {
						let data = *(self.lp_data as *const MSLLHOOKSTRUCT);
						MouseEvent::SetCursor(data.pt.x as i16, data.pt.y as i16)
					},
					// no keys on low order??
					WM_MOUSEWHEEL | WM_MOUSEHWHEEL => unsafe {
						let scroll = (u16_as_i16(get_high_order((*(self.lp_data as *const MSLLHOOKSTRUCT)).mouseData as usize)) / WHEEL_DELTA) as i32;
						if self.param as u32 == WM_MOUSEWHEEL {
							MouseEvent::ScrollVertical(scroll)
						} else {
							MouseEvent::ScrollHorizontal(scroll)
						}
					},
					// No way to trigger??
					WM_MOUSEACTIVATE => return None,
					WM_MOUSEHOVER => return None,
					WM_MOUSELEAVE => return None,
					_ => return None
				}))
			} else {
				None
			}
		} else {
			None
		}
	}
}

pub type CallbackSender = Sender<Callback>;
pub type CallbackReceiver = Receiver<Callback>;

static mut CALLBACKS: Vec<CallbackSender> = Vec::new();
static mut MOUSE_HOOK: HHOOK = null_mut();
static mut KEYBOARD_HOOK: HHOOK = null_mut();

unsafe extern "system" fn mouse_callback(code: c_int, param: WPARAM, lp_data: LPARAM) -> LRESULT {
	handle_callback(code, param, lp_data, MOUSE_HOOK)
}
unsafe extern "system" fn keyboard_callback(code: c_int, param: WPARAM, lp_data: LPARAM) -> LRESULT {
	handle_callback(code, param, lp_data, KEYBOARD_HOOK)
}

unsafe fn handle_callback(code: c_int, param: WPARAM, lp_data: LPARAM, hook: HHOOK) -> LRESULT {
	if code == HC_ACTION {
		for sender in CALLBACKS.iter_mut() {
			sender.send(Callback::new(code, param, lp_data)).ok();
		}
	}
	CallNextHookEx(hook, code, param, lp_data)
}

pub fn init() {
	thread::spawn(|| {
		unsafe {
			KEYBOARD_HOOK = SetWindowsHookExA(WH_KEYBOARD_LL, Some(keyboard_callback), null_mut(), 0);
			MOUSE_HOOK = SetWindowsHookExA(WH_MOUSE_LL, Some(mouse_callback), null_mut(), 0);
			GetMessageA(null_mut(), null_mut(), 0, 0);
		}
	});
}
pub fn get_event_receiver() -> CallbackReceiver {
	let (s, r) = mpsc::channel();
	unsafe {
		CALLBACKS.push(s);
	}
	r
}