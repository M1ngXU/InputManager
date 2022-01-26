use std::fmt::Debug;
use std::mem::transmute;
use winapi::um::winuser::*;
use winapi::shared::minwindef::*;
use crate::sender::{Event, EventQueue};

#[derive(Debug)]
pub enum KeyboardEvent {
	VirtualKeyUp(VirtualKey),
	VirtualKeyDown(VirtualKey),
	Literal(char)
}
impl Event for KeyboardEvent {
	fn into_event_queue(self) -> EventQueue {
		match self {
			KeyboardEvent::VirtualKeyUp(k) => k.into_key_input(KEYEVENTF_KEYUP),
			KeyboardEvent::VirtualKeyDown(k) => k.into_key_input(0),
			KeyboardEvent::Literal(c) => get_keyboard_input(0, c as WORD, KEYEVENTF_UNICODE)
		}.into()
	}
}

pub fn get_keyboard_input(vk: WORD, scan: WORD, dw_flags: DWORD) -> INPUT {
	let mut union: INPUT_u = unsafe { std::mem::zeroed() };
	let inner_union = unsafe { union.ki_mut() };
	*inner_union = KEYBDINPUT {
		wVk: vk,
		wScan: scan,
		dwFlags: dw_flags,
		time: 0,
		dwExtraInfo: 0
	};
	INPUT {
		type_: INPUT_KEYBOARD,
		u: union,
	}
}

/// keys from the [msdn](https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes)
#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum VirtualKey {
	LeftMouseButton = VK_LBUTTON,
	Cancel = VK_CANCEL,
	RightMouseButton = VK_RBUTTON,
	MiddleMouseButton = VK_MBUTTON,
	XButton1 = VK_XBUTTON1,
	XButton2 = VK_XBUTTON2,
	Back = VK_BACK,
	Tab = VK_TAB,
	Clear = VK_CLEAR,
	/// or `Return`
	Enter = VK_RETURN,
	Shift = VK_SHIFT,
	Control = VK_CONTROL,
	Alt = VK_MENU,
	Pause = VK_PAUSE,
	CapsLock = VK_CAPITAL,

	/// or `Hangul`
	ImeKana = VK_KANA,
	ImeOn = 0x16,
	ImeJunja = VK_JUNJA,
	ImeFinal = VK_FINAL,
	/// or `Kanji`
	ImeHanja = VK_HANJA,
	ImeOff = 0x1A,

	Escape = VK_ESCAPE,
	Space = VK_SPACE,
	PageUp = VK_PRIOR,
	PageDown = VK_NEXT,
	End = VK_END,
	Home = VK_HOME,
	ArrowLeft = VK_LEFT,
	ArrowUp = VK_UP,
	ArrowRight = VK_RIGHT,
	ArrowDown = VK_DOWN,

	Select = VK_SELECT,
	Print = VK_PRINT,
	Execute = VK_EXECUTE,
	Snapshot = VK_SNAPSHOT,
	Insert = VK_INSERT,
	Delete = VK_DELETE,
	Help = VK_HELP,

	Kp0 = 0x30,
	Kp1 = 0x31,
	Kp2 = 0x32,
	Kp3 = 0x33,
	Kp4 = 0x34,
	Kp5 = 0x35,
	Kp6 = 0x36,
	Kp7 = 0x37,
	Kp8 = 0x38,
	Kp9 = 0x39,

	A = 0x41,
	B = 0x42,
	C = 0x43,
	D = 0x44,
	E = 0x45,
	F = 0x46,
	G = 0x47,
	H = 0x48,
	I = 0x49,
	J = 0x4A,
	K = 0x4B,
	L = 0x4C,
	M = 0x4D,
	N = 0x4E,
	O = 0x4F,
	P = 0x50,
	Q = 0x51,
	R = 0x52,
	S = 0x53,
	T = 0x54,
	U = 0x55,
	V = 0x56,
	W = 0x57,
	X = 0x58,
	Y = 0x59,
	Z = 0x5A,

	LeftWindows = VK_LWIN,
	RightWindows = VK_RWIN,
	Apps = VK_APPS,
	Sleep = VK_SLEEP,

	NP0 = 0x60,
	NP1 = 0x61,
	NP2 = 0x62,
	NP3 = 0x63,
	NP4 = 0x64,
	NP5 = 0x65,
	NP6 = 0x66,
	NP7 = 0x67,
	NP8 = 0x68,
	NP9 = 0x69,
	Multiply = VK_MULTIPLY,
	Add = VK_ADD,
	Separator = VK_SEPARATOR,
	Subtract = VK_SUBTRACT,
	Decimal = VK_DECIMAL,
	Divide = VK_DIVIDE,

	F1 = VK_F1,
	F2 = VK_F2,
	F3 = VK_F3,
	F4 = VK_F4,
	F5 = VK_F5,
	F6 = VK_F6,
	F7 = VK_F7,
	F8 = VK_F8,
	F9 = VK_F9,
	F10 = VK_F10,
	F11 = VK_F11,
	F12 = VK_F12,
	F13 = VK_F13,
	F14 = VK_F14,
	F15 = VK_F15,
	F16 = VK_F16,
	F17 = VK_F17,
	F18 = VK_F18,
	F19 = VK_F19,
	F20 = VK_F20,
	F21 = VK_F21,
	F22 = VK_F22,
	F23 = VK_F23,
	F24 = VK_F24,

	NumLock = VK_NUMLOCK,
	Scroll = VK_SCROLL,

	LeftShift = VK_LSHIFT,
	RightShift = VK_RSHIFT,
	LeftControl = VK_LCONTROL,
	RightControl = VK_RCONTROL,
	LeftAlt = VK_LMENU,
	RightAlt = VK_RMENU,

	BrowserBack = VK_BROWSER_BACK,
	BrowserForward = VK_BROWSER_FORWARD,
	BrowserRefresh = VK_BROWSER_REFRESH,
	BrowserStop = VK_BROWSER_STOP,
	BrowserSearch = VK_BROWSER_SEARCH,
	BrowserFavorites = VK_BROWSER_FAVORITES,
	BrowserHome = VK_BROWSER_HOME,

	VolumeMute = VK_VOLUME_MUTE,
	VolumeDown = VK_VOLUME_DOWN,
	VolumeUp = VK_VOLUME_UP,
	MediaNextTrack = VK_MEDIA_NEXT_TRACK,
	MediaPreviousTrack = VK_MEDIA_PREV_TRACK,
	MediaPlayPause = VK_MEDIA_PLAY_PAUSE,

	LaunchMail = VK_LAUNCH_MAIL,
	LaunchMediaSelect = VK_LAUNCH_MEDIA_SELECT,
	LaunchApp1 = VK_LAUNCH_APP1,
	LaunchApp2 = VK_LAUNCH_APP2,

	Oem1 = VK_OEM_1,
	OemPlus = VK_OEM_PLUS,
	OemComma = VK_OEM_COMMA,
	OemMinus = VK_OEM_MINUS,
	OemPeriod = VK_OEM_PERIOD,
	Oem2 = VK_OEM_2,
	Oem3 = VK_OEM_3,
	Oem4 = VK_OEM_4,
	Oem5 = VK_OEM_5,
	Oem6 = VK_OEM_6,
	Oem7 = VK_OEM_7,
	Oem8 = VK_OEM_8,
	Oem102 = VK_OEM_102,

	ImeProcessKey = VK_PROCESSKEY,
	Packet = VK_PACKET,
	Attention = VK_ATTN,
	CrSel = VK_CRSEL,
	ExSel = VK_EXSEL,
	EraseEOF = VK_EREOF,
	Play = VK_PLAY,
	Zoom = VK_ZOOM,
	PA1 = VK_PA1,
	OemClear = VK_OEM_CLEAR
}
impl VirtualKey {
	fn into_key_input(self, flags: DWORD) -> INPUT {
		get_keyboard_input(self as u16, 0, flags)
	}
}
impl From<DWORD> for VirtualKey {
	fn from(k: DWORD) -> Self {
		// critical conversion ...
		unsafe { transmute(k) }
	}
}