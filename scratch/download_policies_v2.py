import urllib.request
import os

def download_file(url, save_path):
    headers = {
        'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
    }
    try:
        req = urllib.request.Request(url, headers=headers)
        with urllib.request.urlopen(req, timeout=30) as response:
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

# Trying some different URLs
files_to_download = [
    # Regulation on Medical Insurance Fund Supervision
    ("http://www.moj.gov.cn/pub/sfbgw/zwgkzt/2021ndgzl/202105/P020210517590800000000.pdf", "医疗保障基金使用监督管理条例_司法部.pdf"),
    # Social Insurance Law (trying another source)
    ("http://www.npc.gov.cn/zgrdw/npc/xinwen/2010-10/28/content_1602434.htm", "社会保险法_NPC.html"), # This is HTML, but good for reference
    # Flight Inspection
    ("https://xizang.gov.cn/zwgk/zfxxgk/zfwj/fgwj/202304/P020230419520979854483.pdf", "医疗保障基金飞行检查管理暂行办法_西藏.pdf")
]

for url, filename in files_to_download:
    path = os.path.join(save_dir, filename)
    download_file(url, path)
