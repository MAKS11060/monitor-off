#![allow(unsafe_code)]

use std::os::raw::{c_int, c_uint, c_ulong};

#[link(name = "user32")]
extern "system" {
  fn SendMessageW(hWnd: c_int, Msg: c_uint, wParam: c_uint, lParam: c_ulong) -> c_int;
}

const HWND_BROADCAST: c_int = 0xffff;
const WM_SYSCOMMAND: c_uint = 0x0112;
const SC_MONITORPOWER: c_uint = 0xF170;
const MONITOR_OFF: c_uint = 2;

fn main() {
  unsafe {
    SendMessageW(HWND_BROADCAST, WM_SYSCOMMAND, SC_MONITORPOWER, MONITOR_OFF);
  }
}
