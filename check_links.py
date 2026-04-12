import os
import re
import urllib.parse

def check_links():
    docs_dir = 'docs'
    md_files = []

    for root, dirs, files in os.walk(docs_dir):
        for file in files:
            if file.endswith('.md'):
                md_files.append(os.path.join(root, file))

    link_pattern = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')

    broken_links = 0

    for file_path in md_files:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

            links = link_pattern.findall(content)
            for text, url in links:
                if url.startswith('http') or url.startswith('mailto:'):
                    continue

                # strip anchor and title
                url = url.split('#')[0].strip()
                if ' "' in url:
                    url = url.split(' "')[0].strip()

                if not url: # Was just an anchor
                    continue

                # resolve path relative to current file
                base_dir = os.path.dirname(file_path)
                target_path = os.path.normpath(os.path.join(base_dir, url))

                if not os.path.exists(target_path):
                    print(f"Broken link in {file_path}: [{text}]({url}) -> {target_path}")
                    broken_links += 1

    if broken_links > 0:
        print(f"Found {broken_links} broken links.")
        exit(1)
    else:
        print("All local markdown links are valid.")

if __name__ == '__main__':
    check_links()
