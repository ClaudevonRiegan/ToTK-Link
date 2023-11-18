#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused_macros)]

pub static mut FIGHTER_MANAGER : usize = 0;
pub static mut ITEM_MANAGER : usize = 0;

mod moveset;

static mut CONSTANT_OFFSET : usize = 0x3727390; //13.0.1

use skyline::libc::*;
use std::ffi::CStr;

#[skyline::hook(offset = CONSTANT_OFFSET)]
unsafe fn const_allot_hook(unk: *const u8, constant: *const c_char, mut value: u32) {
    if CStr::from_ptr(constant as _).to_str().unwrap().contains("FIGHTER_LINK_STATUS_KIND_NUM") {
        value = 0x1e9;
    }
    original!()(unk,constant,value)
}

#[skyline::main(name = "totk_link")]
pub fn main() {
    skyline::install_hook!(const_allot_hook);
    moveset::install();
}