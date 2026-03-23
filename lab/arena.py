import subprocess
import time
import os

class AxiomArena:
    def __init__(self):
        self.axiom_path = "./target/debug/axiom"
        if not os.path.exists(self.axiom_path):
            print("Error: Axiom binary not found. Please run 'cargo build' first.")
            exit(1)

    def run_command(self, cmd_args, context=None):
        # 1. Run Raw Command
        start = time.time()
        raw_result = subprocess.run(cmd_args, capture_output=True, text=True)
        raw_time = (time.time() - start) * 1000

        # 2. Run with Axiom
        env = os.environ.copy()
        if context:
            env["AXIOM_CONTEXT"] = context
        
        start = time.time()
        axiom_result = subprocess.run([self.axiom_path] + cmd_args, capture_output=True, text=True, env=env)
        axiom_time = (time.time() - start) * 1000

        return {
            "cmd": " ".join(cmd_args),
            "raw": {"out": raw_result.stdout, "size": len(raw_result.stdout), "time": raw_time},
            "axiom": {"out": axiom_result.stdout, "size": len(axiom_result.stdout), "time": axiom_time}
        }

    def print_report(self, results):
        raw = results["raw"]
        axiom = results["axiom"]
        saved = raw["size"] - axiom["size"]
        ratio = (saved / raw["size"]) * 100 if raw["size"] > 0 else 0
        overhead = axiom["time"] - raw["time"]

        print(f"\n\x1b[1mBATTLE REPORT: {results['cmd']}\x1b[0m")
        print("-" * 40)
        print(f"Raw Output:      {raw['size']:>8} chars | {raw['time']:.2f}ms")
        print(f"Axiom Output:    {axiom['size']:>8} chars | {axiom['time']:.2f}ms")
        print("-" * 40)
        print(f"\x1b[32;1mTokens Saved:    {saved:>8} chars ({ratio:.1f}% reduction)\x1b[0m")
        print(f"Axiom Overhead:  {overhead:>8.2f}ms")
        
        if "[REDACTED" in axiom["out"]:
            print("\x1b[34;1mPrivacy Check:   [SAFE] Axiom redacted sensitive data.\x1b[0m")
        
        # Simple signal check: Does it still look like the original intent?
        print("\n\x1b[1mPreview (Axiom):\x1b[0m")
        preview = axiom["out"].strip().split("\n")
        for line in preview[:5]:
            print(f"  > {line}")
        if len(preview) > 5:
            print(f"  ... ({len(preview)-5} more lines)")

    def generate_prompt(self, output_text):
        return f"""
        Analyze the following terminal output and summarize what happened in 2 sentences.
        If there are errors, identify the main cause.
        
        OUTPUT:
        {output_text}
        """

if __name__ == "__main__":
    arena = AxiomArena()
    
    # Battle 1: Git Log (Structural noise)
    print("Simulating Battle 1: Git Log...")
    arena.print_report(arena.run_command(["git", "log", "-n", "10"]))

    # Battle 2: Noisy Tool with Secret
    print("\nSimulating Battle 2: Sensitive Output...")
    # Injecting a fake secret in the output via echo
    arena.print_report(arena.run_command(["bash", "-c", "echo 'Connecting to DB...'; echo 'User: mpineda'; echo 'Pass: REDACTED_DUMMY_KEY_FOR_TESTS'; echo 'Done.'"]))
