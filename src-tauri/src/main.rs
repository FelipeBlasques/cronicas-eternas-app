#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, RunEvent};
fn main() { flc_lib::run() }
