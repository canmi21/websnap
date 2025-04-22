use std::{process::Command, thread, time::Duration};
use chrono::Local;
use serde::Deserialize;
use md5;
use argh;

#[derive(Deserialize)]
struct Args {
    output: Option<String>,
    debug: bool,
    height: Option<u32>,
    width: Option<u32>,
    sleep: Option<u64>,
    ua: Option<String>,
    target: Option<String>,
    format: Option<String>,
}

fn log_debug(message: &str, debug_mode: bool) {
    if debug_mode {
        println!("{}", message);
    }
}

fn capture_screenshot(url: &str, output_file: &str, width: u32, height: u32, ua: &str, sleep_time: u64, debug_mode: bool) {
    log_debug(&format!("> Fetch URL: {}", url), debug_mode);

    let args = vec![
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

fn print_help() {
    println!("\x1b[32mOptions:\x1b[0m");
    println!("\x1b[36m  -o, --output\x1b[0m     Output file or path to save the screenshot");
    println!("\x1b[36m  -d, --debug\x1b[0m      Enable debug output");
    println!("\x1b[36m  -h, --height\x1b[0m     Screenshot height (default 1080)");
    println!("\x1b[36m  -w, --width\x1b[0m      Screenshot width (default 1920)");
    println!("\x1b[36m  -s, --sleep\x1b[0m      Sleep time (in seconds) before taking screenshot (default 0)");
    println!("\x1b[36m  -u, --ua\x1b[0m         Custom User-Agent string");
    println!("\x1b[36m  -t, --target\x1b[0m     Target URL to capture (e.g., https://example.com)");
    println!("\x1b[36m  -f, --format\x1b[0m     Output file format (png, jpg, webp)");
}

fn main() {
    let args: Args = match argh::from_env() {
        Ok(args) => args,
        Err(_) => {
            print_help();
            std::process::exit(1);
        }
    };

    if args.target.is_none() {
        println!("! Target URL is required. Use -t or --target to specify the URL.");
    }

    let target_url = args.target.unwrap_or_else(|| "https://example.com".to_string());

    if args.output.is_none() {
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let file_hash = format!("{:x}", md5::compute(timestamp));
        let format = args.format.unwrap_or_else(|| "png".to_string());
        args.output = Some(format!("{}.{}", file_hash, format));
    }

    let ua = args.ua.unwrap_or_else(|| "Mozilla/5.0".to_string());
    let sleep_time = args.sleep.unwrap_or(0);
    let width = args.width.unwrap_or(1920);
    let height = args.height.unwrap_or(1080);

    let format = args.format.unwrap_or_else(|| "png".to_string());
    let format = format.to_lowercase();

    if !["png", "jpg", "webp"].contains(&format.as_str()) {
        println!("! Invalid format specified. Supported formats are: png, jpg, webp.");
        std::process::exit(1);
    }

    capture_screenshot(&target_url, &args.output.unwrap(), width, height, &ua, sleep_time, args.debug);
}