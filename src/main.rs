use std::{env, process::Command, thread, time::Duration};
use std::fs::File;
use std::io::Write;
use reqwest::header::{USER_AGENT, HeaderMap};
use chrono::Local;
use serde::Deserialize;

#[derive(Deserialize)]
struct Args {
    output: Option<String>,
    debug: bool,
    height: Option<u32>,
    width: Option<u32>,
    sleep: Option<u64>,
    ua: Option<String>,
    target: String,
}

fn log_debug(message: &str, debug_mode: bool) {
    if debug_mode {
        println!("{}", message);
    }
}

fn capture_screenshot(url: &str, output_file: &str, width: u32, height: u32, ua: &str, sleep_time: u64, debug_mode: bool) {
    log_debug(&format!("> Fetch URL: {}", url), debug_mode);

    let mut args = vec![
        "--headless".to_string(),
        "--disable-gpu".to_string(),
        "--no-sandbox".to_string(),
        format!("--window-size={},{}", width, height),
        format!("--user-agent={}", ua),
    ];

    let status = Command::new("google-chrome-stable")
        .args(args)
        .arg(format!("--screenshot={}", output_file))
        .arg(url)
        .status()
        .expect("Failed to execute process");

    if status.success() {
        log_debug(&format!("+ Screenshot saved to {}", output_file), debug_mode);
    } else {
        log_debug("! Error capturing screenshot", debug_mode);
        std::process::exit(1);
    }

    if sleep_time > 0 {
        log_debug(&format!("- Sleeping for {} seconds", sleep_time), debug_mode);
        thread::sleep(Duration::new(sleep_time, 0));
    }
}

fn main() {
    let args = Args {
        output: Some("screenshot.png".to_string()),
        debug: true,
        height: Some(1080),
        width: Some(1920),
        sleep: Some(0),
        ua: Some("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36".to_string()),
        target: "https://example.com".to_string(),
    };

    if args.target.is_empty() {
        eprintln!("! Target URL is required.");
        std::process::exit(1);
    }

    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let file_hash = format!("{:x}", md5::compute(timestamp));

    let output = args.output.unwrap_or_else(|| format!("{}.png", file_hash));
    let ua = args.ua.unwrap_or_else(|| "Mozilla/5.0".to_string());

    capture_screenshot(&args.target, &output, args.width.unwrap_or(1920), args.height.unwrap_or(1080), &ua, args.sleep.unwrap_or(0), args.debug);
}