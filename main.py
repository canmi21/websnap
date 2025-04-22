from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service

def capture_screenshot(url, output_file, width, height):
    chrome_options = Options()
    chrome_options.add_argument('--headless')
    chrome_options.add_argument('--no-sandbox')
    chrome_options.add_argument('--disable-dev-shm-usage')

    service = Service(ChromeDriverManager().install())

    driver = webdriver.Chrome(service=service, options=chrome_options)
    driver.set_window_size(width, height)
    driver.get(url)
    driver.save_screenshot(output_file)
    driver.quit()
    print(f"Screenshot saved to {output_file}")

if __name__ == '__main__':
    url = input("Enter URL: ")
    output_file = input("Enter output filename (default 'screenshot.png'): ") or 'screenshot.png'
    width = int(input("Enter screenshot width (default 1920): ") or 1920)
    height = int(input("Enter screenshot height (default 1080): ") or 1080)

    capture_screenshot(url, output_file, width, height)