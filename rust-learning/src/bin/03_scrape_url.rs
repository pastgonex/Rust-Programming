/*
 * @Author: Binqi Ni
 * @Date: 2021-09-11 00:39:36
 * @LastEditTime: 2021-10-02 08:14:57
 * @LastEditors: Binqi Ni
 * @FilePath: /Rust-Programming/rust-learning/src/bin/03_scrape_url.rs
 */

use std::fs;

// main函数现在返回一个Result
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<String>>();
    let url = &args[1];
    let output = &args[2];
    println!("{}", output);
    println!("Fetching url: {}", url);
    let body = reqwest::blocking::get(url)?.text()?;

    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(&output, md.as_bytes())?;
    println!("Converted markdown has been saved in {}.", output);
    Ok(())
}
