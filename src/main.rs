mod app;
mod calculator;
mod models;
use leptos::prelude::mount_to_body;
mod components;
use crate::app::App;
fn main() {
    println!("=== 缺氧 (Oxygen Not Included) 火箭航程计算器测试 ===\n");
    mount_to_body(App);
}
