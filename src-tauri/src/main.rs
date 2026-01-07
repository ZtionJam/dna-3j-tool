// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use jjj_core::*;
fn main() {
    dna_3j_tool_lib::run()
}
