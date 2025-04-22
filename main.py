import argparse
import time
import sys
import hashlib
from datetime import datetime
from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service
import platform

def log_debug(message, debug_mode):
    if debug_mode:
        print(f"{message}")

def capture_screenshot(url, output_file, width, height, ua, sleep_time, debug_mode):
    chrome_options = Options()
    chrome_options.add_argument('--headless')
    chrome_options.add_argument('--no-sandbox')
    chrome_options.add_argument('--disable-dev-shm-usage')
    
    if ua:
        chrome_options.add_argument(f"user-agent={ua}")

    service = Service(ChromeDriverManager().install())
    
    try:
        log_debug(f"> Fetch URL: {url}", debug_mode)

        driver = webdriver.Chrome(service=service, options=chrome_options)
        
        driver.set_window_size(width, height)
        driver.get(url)

        if sleep_time:
            log_debug(f"- Sleeping for {sleep_time} seconds", debug_mode)
            time.sleep(sleep_time)

        driver.save_screenshot(output_file)
        driver.quit()

        log_debug(f"+ Screenshot saved to {output_file}", debug_mode)
    except Exception as e:
        log_debug(f"! Error: {str(e)}", debug_mode)
        sys.exit(1)

def print_help():
    print("\033[32mOptions:\033[0m")
    print("\033[36m  -o, --output\033[0m     Output file or path to save the screenshot")
    print("\033[36m  -d, --debug\033[0m      Enable debug output")
    print("\033[36m  -h, --height\033[0m     Screenshot height (default 1080)")
    print("\033[36m  -w, --width\033[0m      Screenshot width (default 1920)")
    print("\033[36m  -s, --sleep\033[0m      Sleep time (in seconds) before taking screenshot (default 0)")
    print("\033[36m  -u, --ua\033[0m         Custom User-Agent string")
    print("\033[36m  -t, --target\033[0m     Target URL to capture (e.g., https://example.com)")

def main():
    parser = argparse.ArgumentParser(description="Capture a screenshot of a webpage.", add_help=False)
    
    parser.add_argument('-o', '--output', type=str, help="Output file or path to save the screenshot")
    parser.add_argument('-d', '--debug', action='store_true', help="Enable debug output")
    parser.add_argument('-h', '--height', type=int, default=1080, help="Screenshot height (default 1080)")
    parser.add_argument('-w', '--width', type=int, default=1920, help="Screenshot width (default 1920)")
    parser.add_argument('-s', '--sleep', type=int, default=0, help="Sleep time (in seconds) before taking screenshot (default 0)")
    parser.add_argument('-u', '--ua', type=str, help="Custom User-Agent string")
    parser.add_argument('-t', '--target', type=str, help="Target URL to capture (e.g., https://example.com)")

    args = parser.parse_args()

    if not args.target:
        print("! Target URL is required. Use -t or --target to specify the URL.")
        sys.exit(1)

    if not args.output:
        timestamp = datetime.now().strftime("%Y%m%d%H%M%S")
        hash_object = hashlib.md5(timestamp.encode())
        file_hash = hash_object.hexdigest()
        args.output = f"{file_hash}.png"

    if not args.ua:
        args.ua = f"Mozilla/5.0 ({platform.system()}; {platform.release()}) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.36"

    capture_screenshot(args.target, args.output, args.width, args.height, args.ua, args.sleep, args.debug)

if __name__ == '__main__':
    main()