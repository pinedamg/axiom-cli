import os
import re

def verify_markdown_links(directory="docs"):
    broken_links = []
    link_pattern = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')

    for root, _, files in os.walk(directory):
        for file in files:
            if file.endswith('.md'):
                filepath = os.path.join(root, file)
                with open(filepath, 'r', encoding='utf-8') as f:
                    content = f.read()

                    matches = link_pattern.findall(content)
                    for text, url in matches:
                        if url.startswith('http') or url.startswith('#') or url.startswith('mailto:'):
                            continue

                        # Strip anchors and titles
                        clean_url = url.split('#')[0].strip()
                        if ' "' in clean_url:
                            clean_url = clean_url.split(' "')[0]
                        if not clean_url:
                            continue

                        # Resolve path relative to the current file's directory
                        target_path = os.path.normpath(os.path.join(root, clean_url))

                        if not os.path.exists(target_path):
                            broken_links.append((filepath, url, target_path))

    if broken_links:
        print("Broken links found:")
        for fp, url, target in broken_links:
            print(f"- In {fp}: '{url}' (Resolved to: {target})")
        return False
    else:
        print("All local markdown links are valid.")
        return True

if __name__ == "__main__":
    success = verify_markdown_links()
    if not success:
        exit(1)
