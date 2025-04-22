use clap::{App, Arg};  // Import the new clap syntax
use std::{process::Command, thread, time::Duration};
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
    target: Option<String>,
    dist: Option<String>,
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
    println!("\x1b[36m  -d, --dist\x1b[0m       Output file format (png, webp, jpg)");
}

fn main() {
    let matches = App::new("WebSnap")
        .version("1.0")
        .about("Capture screenshots of websites")
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .takes_value(true)
            .help("Output file or path to save the screenshot"))
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .takes_value(false)
            .help("Enable debug output"))
        .arg(Arg::new("height")
            .short('h')
            .long("height")
            .takes_value(true)
            .default_value("1080")
            .help("Screenshot height"))
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .takes_value(true)
            .default_value("1920")
            .help("Screenshot width"))
        .arg(Arg::new("sleep")
            .short('s')
            .long("sleep")
            .takes_value(true)
            .default_value("0")
            .help("Sleep time before taking screenshot"))
        .arg(Arg::new("ua")
            .short('u')
            .long("ua")
            .takes_value(true)
            .default_value("Mozilla/5.0")
            .help("Custom User-Agent string"))
        .arg(Arg::new("target")
            .short('t')
            .long("target")
            .takes_value(true)
            .help("Target URL to capture"))
        .arg(Arg::new("dist")
            .short('d')
            .long("dist")
            .takes_value(true)
            .default_value("png")
            .help("Output file format"))
        .get_matches();

    // Parsing arguments
    let output = matches.value_of("output").unwrap_or_default();
    let debug = matches.is_present("debug");
    let height = matches.value_of("height").unwrap_or("1080").parse::<u32>().unwrap();
    let width = matches.value_of("width").unwrap_or("1920").parse::<u32>().unwrap();
    let sleep = matches.value_of("sleep").unwrap_or("0").parse::<u64>().unwrap();
    let ua = matches.value_of("ua").unwrap_or("Mozilla/5.0");
    let target = matches.value_of("target").unwrap_or("https://example.com");
    let dist = matches.value_of("dist").unwrap_or("png");

    capture_screenshot(target, output, width, height, ua, sleep, debug);
}