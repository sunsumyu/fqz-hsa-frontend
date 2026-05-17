import urllib.request
import os
import ssl

def download_file(url, save_path, referer):
    # Ignore SSL certificate verification
    ctx = ssl.create_default_context()
    ctx.check_hostname = False
    ctx.verify_mode = ssl.CERT_NONE

    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
        'Referer': referer
    }
    try:
        req = urllib.request.Request(url, headers=headers)
        with urllib.request.urlopen(req, timeout=30, context=ctx) as response:
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

# NHSA Download Links
files_to_download = [
    # 医疗保障基金使用监督管理条例实施细则 (2026)
    ("http://www.nhsa.gov.cn/module/download/down.jsp?i_ID=11166&colID=173", "医疗保障基金使用监督管理条例实施细则_2026.doc", "http://www.nhsa.gov.cn/art/2026/2/13/art_173_12117.html"),
    # 医疗保障基金飞行检查管理暂行办法
    ("http://www.nhsa.gov.cn/module/download/down.jsp?i_ID=10991&colID=173", "医疗保障基金飞行检查管理暂行办法.doc", "http://www.nhsa.gov.cn/art/2023/3/13/art_173_10255.html"),
    # 医疗保障行政处罚程序暂行规定
    ("http://www.nhsa.gov.cn/module/download/down.jsp?i_ID=10989&colID=173", "医疗保障行政处罚程序暂行规定.doc", "http://www.nhsa.gov.cn/art/2021/6/3/art_173_5228.html"),
    # 基本医疗保险用药管理暂行办法
    ("http://www.nhsa.gov.cn/module/download/down.jsp?i_ID=10977&colID=173", "基本医疗保险用药管理暂行办法.doc", "http://www.nhsa.gov.cn/art/2020/7/31/art_173_3419.html")
]

for url, filename, referer in files_to_download:
    path = os.path.join(save_dir, filename)
    download_file(url, path, referer)
