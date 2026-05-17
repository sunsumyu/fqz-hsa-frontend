import urllib.request
import os
import ssl

def download_file(url, save_path):
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE

    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
    }
    try:
        req = urllib.request.Request(url, headers=headers)
        with urllib.request.urlopen(req, timeout=60, context=ctx) as response:
            with open(save_path, 'wb') as f:
                f.write(response.read())
        print(f"Successfully downloaded {save_path}")
        return True
    except Exception as e:
        print(f"Failed to download {url}: {e}")
        return False

save_dir = r"e:\chain\fqz-hsa-frontend\data\knowledge_base\files"
if not os.path.exists(save_dir):
    os.makedirs(save_dir)

# Final Regulation PDF from Guizhou Gov
url = "https://wjw.guizhou.gov.cn/ztzl_500663/qwpfzl/202103/P020210310395647378096.pdf"
path = os.path.join(save_dir, "医疗保障基金使用监督管理条例_2021.pdf")
download_file(url, path)
