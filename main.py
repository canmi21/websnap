from selenium import webdriver
from webdriver_manager.chrome import ChromeDriverManager
from selenium.webdriver.chrome.options import Options
from selenium.webdriver.chrome.service import Service

def capture_screenshot(url, output_file):
    chrome_options = Options()
    chrome_options.add_argument('--headless')
    chrome_options.add_argument('--no-sandbox')
    chrome_options.add_argument('--disable-dev-shm-usage')

    service = Service(ChromeDriverManager().install())

    driver = webdriver.Chrome(service=service, options=chrome_options)
    driver.get(url)
    driver.save_screenshot(output_file)
    driver.quit()
    print(f"Screenshot saved to {output_file}")

if __name__ == '__main__':
    url = input("Enter URL: ")
    output_file = input("Enter output filename (default 'screenshot.png'): ") or 'screenshot.png'
    capture_screenshot(url, output_file)