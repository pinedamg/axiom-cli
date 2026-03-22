import json
import os
import subprocess
import time
from urllib import request, error

def load_env():
    env = {}
    if os.path.exists(".env"):
        with open(".env") as f:
            for line in f:
                if "=" in line:
                    key, value = line.strip().split("=", 1)
                    env[key.strip()] = value.strip().replace('"', '').replace("'", "")
    return env

def call_groq(prompt, api_key):
    url = "https://api.groq.com/openai/v1/chat/completions"
    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json",
        "User-Agent": "AxiomArena/1.0"
    }
    data = {
        "model": "llama-3.1-8b-instant",
        "messages": [{"role": "user", "content": prompt}]
    }
    
    req = request.Request(url, data=json.dumps(data).encode("utf-8"), headers=headers)
    try:
        with request.urlopen(req) as response:
            res_body = response.read().decode("utf-8")
            return json.loads(res_body)
    except error.HTTPError as e:
        error_msg = e.read().decode('utf-8')
        print(f"Error calling Groq: {e.code} - {error_msg}")
        return None
    except Exception as e:
        print(f"Connection Error: {str(e)}")
        return None

def run_battle():
    env_vars = load_env()
    api_key = env_vars.get("GROQ_API_KEY")
    if not api_key:
        print("Error: GROQ_API_KEY not found in .env")
        return

    # Build the noisy output
    noisy_output = ""
    for i in range(25):
        noisy_output += f"[SUCCESS] Module {i:03} compiled in {10+i}ms\n"
    noisy_output += "[ERROR] src/auth.rs:12: 'User' struct has no field 'api_key'\n"
    for i in range(5):
        noisy_output += f"[SUCCESS] Cleanup sequence {i} ok\n"

    # 1. Axiom Processing
    axiom_path = "./target/debug/axiom"
    process = subprocess.run(
        [axiom_path, "bash", "-c", f"echo '{noisy_output}'"],
        capture_output=True, text=True,
        env={**os.environ, "AXIOM_CONTEXT": "Find why the build is failing"}
    )
    axiom_output = process.stdout

    # 2. The Battle
    print("\x1b[1m--- AXIOM SEMANTIC BATTLE (v2) ---\x1b[0m")
    
    print("\n[1/2] Asking Groq about RAW output...")
    prompt_raw = f"Task: Summarize the output and identify errors.\nOutput:\n{noisy_output}"
    res_raw = call_groq(prompt_raw, api_key)
    
    print("[2/2] Asking Groq about AXIOM output...")
    prompt_axiom = f"Task: Summarize the output and identify errors.\nOutput:\n{axiom_output}"
    res_axiom = call_groq(prompt_axiom, api_key)

    # 3. Report
    if res_raw and res_axiom:
        tokens_raw = res_raw['usage']['prompt_tokens']
        tokens_axiom = res_axiom['usage']['prompt_tokens']
        summary_axiom = res_axiom['choices'][0]['message']['content']

        print("\n" + "="*50)
        print(f"\x1b[1mBATTLE RESULTS\x1b[0m")
        print(f"RAW Tokens:     {tokens_raw:>5}")
        print(f"AXIOM Tokens:   {tokens_axiom:>5}")
        saved = tokens_raw - tokens_axiom
        reduction = (saved / tokens_raw * 100)
        print(f"\x1b[32;1mSAVINGS:        {saved:>5} tokens ({reduction:.1f}% reduction)\x1b[0m")
        print("="*50)
        
        print("\n\x1b[1mGroq Conclusion (using AXIOM output):\x1b[0m")
        print(f"{summary_axiom.strip()}")
    else:
        print("\nBattle failed due to connection/API issues.")

if __name__ == "__main__":
    run_battle()
