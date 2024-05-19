#![no_main]
#![no_std]

// Required for panic handler
extern crate alloc;
extern crate flipperzero_alloc;
extern crate flipperzero_rt;

use alloc::ffi::CString;
use core::ffi::CStr;
use flipperzero::{
    dialogs::{DialogMessage, DialogMessageButton, DialogsApp},
    gui::canvas::Align,
};
use flipperzero_rt::{entry, manifest};

// Define the FAP Manifest for this application
manifest!(
    name = "Flipper Zero My Card",
    app_version = 1,
    has_icon = true,
    // See https://github.com/flipperzero-rs/flipperzero/blob/v0.7.2/docs/icons.md for icon format
    icon = "rustacean-10x10.icon",
);

// Define the entry function
entry!(main);

const BUTTON_RIGHT: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"Next\0") };
const BUTTON_LEFT: &'static CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"Prev\0") };

const PAGE_COUNT: u8 = 3;

// Entry point
fn main(_args: *mut u8) -> i32 {
    let mut cur_page: u8 = 0;

    let page_0 = CString::new(*b"DEADBLACKCLOVER").unwrap();
    let page_1 =
        CString::new(*b"FP Hacker\nPL nerd\nDecentralized enthusiast").unwrap();
    let page_2 = CString::new(*b"deadblackclover.net").unwrap();

    let mut dialogs = DialogsApp::open();
    let mut message = DialogMessage::new();

    message.set_buttons(Some(BUTTON_LEFT), None, Some(BUTTON_RIGHT));

    loop {
        match cur_page {
            0 => message.set_text(&page_0, 64, 32, Align::Center, Align::Center),
            1 => message.set_text(&page_1, 64, 32, Align::Center, Align::Center),
            2 => message.set_text(&page_2, 64, 32, Align::Center, Align::Center),
            _ => (),
        }
        match dialogs.show_message(&message) {
            DialogMessageButton::Left => {
                if cur_page > 0 {
                    cur_page -= 1;
                } else {
                    cur_page = PAGE_COUNT - 1;
                }
            }
            DialogMessageButton::Right => {
                if cur_page + 1 < PAGE_COUNT {
                    cur_page += 1;
                } else {
                    cur_page = 0;
                }
            }
            DialogMessageButton::Back => return 0,
            DialogMessageButton::Center => (),
        }
    }
}
