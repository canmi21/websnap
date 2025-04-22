use headless_chrome::{Browser, LaunchOptions};
use std::{fs, path::Path, time::Duration};
use clap::{Arg, Command};
use chrono::Utc;
use md5;

fn log_debug(message: &str, debug_mode: bool) {
    if debug_mode {
        println!("{}", message);
    }
}

fn capture_screenshot(url: &str, output_file: &str, width: i32, height: i32, ua: &str, sleep_time: u64, debug_mode: bool) -> Result<(), Box<dyn std::error::Error>> {
    log_debug(&format!("> Fetch URL: {}", url), debug_mode);

    let options = LaunchOptions {
        headless: true,
        window_size: Some((width, height)),
        args: vec!["--no-sandbox", "--disable-dev-shm-usage", &format!("--user-agent={}", ua)],
        ..Default::default()
    };

    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    tab.navigate_to(url)?;
    
    if sleep_time > 0 {
        log_debug(&format!("- Sleeping for {} seconds", sleep_time), debug_mode);
        std::thread::sleep(Duration::from_secs(sleep_time));
    }

    let screenshot = tab.capture_screenshot(headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Png, None, None, true)?;
    fs::write(output_file, screenshot)?;
    
    log_debug(&format!("+ Screenshot saved to {}", output_file), debug_mode);
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("webpage_screenshot")
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
            .value_name("HEIGHT")
            .default_value("1080")
            .help("Screenshot height"))
        .arg(Arg::new("width")
            .short('w')
            .long("width")
            .value_name("WIDTH")
            .default_value("1920")
            .help("Screenshot width"))
        .arg(Arg::new("sleep")
            .short('s')
            .long("sleep")
            .value_name("SECONDS")
            .default_value("0")
            .help("Sleep time before taking screenshot"))
        .arg(Arg::new("ua")
            .short('u')
            .long("ua")
            .value_name("USER_AGENT")
            .help("Custom User-Agent string"))
        .arg(Arg::new("target")
            .short('t')
            .long("target")
            .value_name("URL")
            .help("Target URL to capture")
            .required(true))
        .get_matches();

    let target = matches.get_one::<String>("target").unwrap();
    let debug = matches.get_flag("debug");
    let width: i32 = matches.get_one::<String>("width").unwrap().parse()?;
    let height: i32 = matches.get_one::<String>("height").unwrap().parse()?;
    let sleep: u64 = matches.get_one::<String>("sleep").unwrap().parse()?;
    
    let output = match matches.get_one::<String>("output") {
        Some(o) => o.to_string(),
        None => {
            let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
            let digest = md5::compute(timestamp.as_bytes());
            format!("{:x}.png", digest)
        }
    };

    let ua = matches.get_one::<String>("ua").map_or_else(
        || format!("Mozilla/5.0 ({}; {}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36", 
            std::env::consts::OS, std::env::consts::OS),
        |s| s.to_string()
    );

    if let Err(e) = capture_screenshot(target, &output, width, height, &ua, sleep, debug) {
        log_debug(&format!("! Error: {}", e), debug);
        std::process::exit(1);
    }

    Ok(())
}