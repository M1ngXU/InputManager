use std::mem::transmute;

pub fn i32_to_u32(n: i32) -> u32 {
	unsafe {
		transmute(n)
	}
}

pub fn u16_as_i16(n: u16) -> i16 {
	unsafe {
		transmute(n)
	}
}

#[allow(dead_code)]
pub fn get_low_order(n: usize) -> u16 {
	(n & u16::MAX as usize) as u16
}

pub fn get_high_order(n: usize) -> u16 {
	n.checked_shr(16).unwrap() as u16
}