use std::{process::Command, thread, time::Duration};
use chrono::Local;
use serde::Deserialize;
use md5;
use clap::{Arg, Command as ClapCommand};

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

pub fn capture_screenshot(url: &str, output_file: &str, width: u32, height: u32, ua: &str, sleep_time: u64, debug_mode: bool) {
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
    println!("\x1b[36m  -f, --format\x1b[0m     Output file format (webp, png, jpg)");
}

fn main() {
    let matches = ClapCommand::new("websnap")
        .version("1.0")
        .about("Capture screenshots of websites")
        .disable_help_flag(true)
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output file or path to save the screenshot"))
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .action(clap::ArgAction::SetTrue)
            .help("Enable debug output"))
        .arg(Arg::new("height")
            .short('h')
            .long("height")
            .value_name("PIXELS")
            .help("Screenshot height (default 1080)"))
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .value_name("PIXELS")
            .help("Screenshot width (default 1920)"))
        .arg(Arg::new("sleep")
            .short('s')
            .long("sleep")
            .value_name("SECONDS")
            .help("Sleep time (in seconds) before taking screenshot (default 0)"))
        .arg(Arg::new("ua")
            .short('u')
            .long("ua")
            .value_name("STRING")
            .help("Custom User-Agent string"))
        .arg(Arg::new("target")
            .short('t')
            .long("target")
            .value_name("URL")
            .help("Target URL to capture (e.g., https://example.com)"))
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .value_name("FORMAT")
            .help("Output file format (png, webp, jpg)"))
        .arg(Arg::new("help")
            .long("help")
            .action(clap::ArgAction::SetTrue)
            .help("Print help information"))
        .get_matches();

    if matches.get_flag("help") || std::env::args().len() == 1 {
        print_help();
        std::process::exit(0);
    }

    let args = Args {
        output: matches.get_one::<String>("output").cloned(),
        debug: matches.get_flag("debug"),
        height: matches.get_one::<String>("height").and_then(|s| s.parse().ok()),
        width: matches.get_one::<String>("width").and_then(|s| s.parse().ok()),
        sleep: matches.get_one::<String>("sleep").and_then(|s| s.parse().ok()),
        ua: matches.get_one::<String>("ua").cloned(),
        target: matches.get_one::<String>("target").cloned(),
        format: matches.get_one::<String>("format").cloned(),
    };

    let has_other_args = args.output.is_some() || args.debug || args.height.is_some() || args.width.is_some() ||
                         args.sleep.is_some() || args.ua.is_some() || args.format.is_some();

    if args.target.is_none() && has_other_args {
        println!("! Target URL is required. Use -t or --target to specify the URL.");
        let target_url = "https://example.com".to_string();
        let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
        let file_hash = format!("{:x}", md5::compute(timestamp));
        let output = args.output.unwrap_or_else(|| format!("{}.{}", file_hash, args.format.as_deref().unwrap_or("webp")));
        let ua = args.ua.unwrap_or_else(|| "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36".to_string());
        capture_screenshot(&target_url, &output, args.width.unwrap_or(1920), args.height.unwrap_or(1080), &ua, args.sleep.unwrap_or(0), args.debug);
        std::process::exit(1);
    }

    let target_url = args.target.unwrap_or("https://example.com".to_string());
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let file_hash = format!("{:x}", md5::compute(timestamp));
    let output = args.output.unwrap_or_else(|| format!("{}.{}", file_hash, args.format.as_deref().unwrap_or("webp")));
    let ua = args.ua.unwrap_or_else(|| "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36".to_string());

    capture_screenshot(&target_url, &output, args.width.unwrap_or(1920), args.height.unwrap_or(1080), &ua, args.sleep.unwrap_or(0), args.debug);
}