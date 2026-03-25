import os
import re

def check_links():
    docs_dir = 'docs'
    md_files = []
    for root, _, files in os.walk(docs_dir):
        for file in files:
            if file.endswith('.md'):
                md_files.append(os.path.join(root, file))

    link_pattern = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')
    bad_links = 0

    for md_file in md_files:
        with open(md_file, 'r', encoding='utf-8') as f:
            content = f.read()
            for match in link_pattern.finditer(content):
                url = match.group(2)
                # Ignore absolute urls and anchors
                if url.startswith('http') or url.startswith('#') or url.startswith('mailto:'):
                    continue

                # Clean up anchor from relative URL
                url_path = url.split('#')[0]
                if not url_path:
                    continue

                target_path = os.path.normpath(os.path.join(os.path.dirname(md_file), url_path))
                if not os.path.exists(target_path):
                    print(f"Broken link in {md_file}: {url} -> {target_path}")
                    bad_links += 1

    if bad_links > 0:
        print(f"Found {bad_links} broken links.")
        return False
    else:
        print("All relative links verified successfully.")
        return True

if __name__ == "__main__":
    if not check_links():
        exit(1)
