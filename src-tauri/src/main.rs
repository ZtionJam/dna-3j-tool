// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use  jjj_core::hello;
fn main() {
    hello();
    dna_3j_tool_lib::run()
}
