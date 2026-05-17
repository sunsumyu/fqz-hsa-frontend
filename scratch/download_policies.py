import os
import requests

def download_file(url, save_path):
    try:
        response = requests.get(url, stream=True, timeout=30)
        response.raise_for_status()
        with open(save_path, 'wb') as f:
            for chunk in response.iter_content(chunk_size=8192):
                f.write(chunk)
        print(f"Successfully downloaded {save_path}")
        return True
    except Exception as e:
        print(f"Failed to download {url}: {e}")
        return False

save_dir = r"e:\chain\fqz-hsa-frontend\data\knowledge_base\files"
if not os.path.exists(save_dir):
    os.makedirs(save_dir)

files_to_download = [
    ("https://www.moj.gov.cn/pub/sfbgw/zwgkzt/2021ndgzl/202105/P020210517590800000000.pdf", "医疗保障基金使用监督管理条例.pdf"),
    ("https://www.kaizencpa.com/files/PDF/Laws_PRC_Social_Insurance_Law.pdf", "中华人民共和国社会保险法.pdf"),
    ("https://xizang.gov.cn/zwgk/zfxxgk/zfwj/fgwj/202304/P020230419520979854483.pdf", "医疗保障基金飞行检查管理暂行办法.pdf"),
    ("https://www.beijing.gov.cn/zhengce/zhengcefagui/202008/P020200812586737525252.pdf", "基本医疗保险用药管理暂行办法.pdf")
]

for url, filename in files_to_download:
    path = os.path.join(save_dir, filename)
    download_file(url, path)
