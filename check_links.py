import os
import re

def check_links():
    docs_dir = 'docs'
    md_files = []

    for root, _, files in os.walk(docs_dir):
        for file in files:
            if file.endswith('.md'):
                md_files.append(os.path.join(root, file))

    link_pattern = re.compile(r'\[.*?\]\((.*?)\)')

    broken_links = 0
    for md_file in md_files:
        with open(md_file, 'r', encoding='utf-8') as f:
            content = f.read()

        links = link_pattern.findall(content)

        for link in links:
            # Skip external links
            if link.startswith('http://') or link.startswith('https://'):
                continue

            # Strip anchor and title
            target = link.split('#')[0].strip()
            target = target.split(' "')[0].strip()

            if not target: # it was just an anchor
                continue

            # Calculate absolute path relative to the repo
            file_dir = os.path.dirname(md_file)
            target_path = os.path.normpath(os.path.join(file_dir, target))

            if not os.path.exists(target_path):
                print(f"Broken link in {md_file}: {link} (resolved to {target_path})")
                broken_links += 1

    if broken_links == 0:
        print("All local markdown links are functional.")
        return 0
    else:
        print(f"Found {broken_links} broken links.")
        return 1

if __name__ == "__main__":
    exit(check_links())
