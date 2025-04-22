import argparse
import time
from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service

def log_debug(message, debug_mode):
    """Log messages when debug mode is enabled."""
    if debug_mode:
        print(f"! {message}")

def capture_screenshot(url, output_file, width, height, ua, sleep_time, debug_mode):
    chrome_options = Options()
    chrome_options.add_argument('--headless')
    chrome_options.add_argument('--no-sandbox')
    chrome_options.add_argument('--disable-dev-shm-usage')
    
    if ua:
        chrome_options.add_argument(f"user-agent={ua}")

    service = Service(ChromeDriverManager().install())
    
    log_debug(f"> Loading URL: {url}", debug_mode)

    driver = webdriver.Chrome(service=service, options=chrome_options)
    
    driver.set_window_size(width, height)
    driver.get(url)

    if sleep_time:
        log_debug(f"- Sleeping for {sleep_time} seconds", debug_mode)
        time.sleep(sleep_time)

    driver.save_screenshot(output_file)
    driver.quit()

    log_debug(f"+ Screenshot saved to {output_file}", debug_mode)

def main():
    parser = argparse.ArgumentParser(description="Capture a screenshot of a webpage.")
    
    parser.add_argument('-o', '--output', type=str, required=True, help="Output file or path to save the screenshot")
    parser.add_argument('-d', '--debug', action='store_true', help="Enable debug output")
    parser.add_argument('-h', '--height', type=int, default=1080, help="Screenshot height (default 1080)")
    parser.add_argument('-w', '--width', type=int, default=1920, help="Screenshot width (default 1920)")
    parser.add_argument('-s', '--sleep', type=int, default=0, help="Sleep time (in seconds) before taking screenshot (default 0)")
    parser.add_argument('-u', '--ua', type=str, help="Custom User-Agent string")

    args = parser.parse_args()

    url = input("Enter URL: ")

    log_debug(f"> Starting screenshot capture process", args.debug)

    capture_screenshot(url, args.output, args.width, args.height, args.ua, args.sleep, args.debug)

if __name__ == '__main__':
    main()