import urllib.request
import os
import ssl

def download_file(url, save_path, referer):
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE

    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
        'Referer': referer
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

files_to_download = [
    # 医疗保障基金使用监督管理条例实施细则 (2026)
    ("https://www.nhsa.gov.cn/attach/0/d36d1c63551841f9bd94b01c52c5d068.pdf", "医疗保障基金使用监督管理条例实施细则_2026.pdf", "https://www.nhsa.gov.cn/art/2026/2/13/art_173_19681.html"),
    # 医疗保障基金使用监督管理条例 (2021)
    ("https://www.nhsa.gov.cn/module/download/down.jsp?i_ID=4474&colID=175", "医疗保障基金使用监督管理条例_2021.doc", "https://www.nhsa.gov.cn/art/2021/2/19/art_175_4474.html")
]

for url, filename, referer in files_to_download:
    path = os.path.join(save_dir, filename)
    download_file(url, path, referer)
