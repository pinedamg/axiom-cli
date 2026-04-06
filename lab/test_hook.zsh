# Axiom Stealth Hook (Zsh)
# Source this file to test the dynamic interception: source lab/test_hook.zsh

# 1. Tools we want to intercept automatically
AXIOM_INTERCEPT_TOOLS=("git" "npm" "cargo" "docker" "kubectl" "terraform" "ls" "ps" "rg" "go")

# 2. Tools we MUST NEVER intercept (Interactive/Complex)
AXIOM_EXCLUDE_TOOLS=("vi" "vim" "nano" "ssh" "top" "htop" "axiom" "man" "less" "watch")

# The magic function
axiom_preexec_hook() {
    local cmd_full="$1"
    local cmd_base=$(echo "$cmd_full" | awk '{print $1}')

    # Check if we should skip
    for exclude in "${AXIOM_EXCLUDE_TOOLS[@]}"; do
        if [[ "$cmd_base" == "$exclude" ]]; then
            return
        fi
    done

    # Check if we should intercept
    for tool in "${AXIOM_INTERCEPT_TOOLS[@]}"; do
        if [[ "$cmd_base" == "$tool" ]]; then
            # Re-write the command to use axiom internally
            # Note: This is a simplified version for Zsh preexec
            # In a real installer, we might use a different approach (like a shell function)
            echo "\x1b[2m[axiom] Intercepting: $cmd_full\x1b[0m"
            # In Zsh, we can't easily change the command in preexec without 'set -o DEBUG' 
            # or a complex 'preexec' trap. The best way for the CLI is to use ALIASES or SHIMS.
            # But we can make a dynamic function instead of an alias!
        fi
    done
}

# The Best Way: Dynamic Function Mapping
for tool in "${AXIOM_INTERCEPT_TOOLS[@]}"; do
    eval "$tool() { axiom $tool \"\$@\"; }"
done

echo "\x1b[32m✅ Axiom Dynamic Hook loaded for: ${AXIOM_INTERCEPT_TOOLS[*]}\x1b[0m"
echo "\x1b[2m(Try running 'ls' or 'git status' now)\x1b[0m"
