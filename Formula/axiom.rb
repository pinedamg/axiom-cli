class Axiom < Formula
  desc "The Semantic Token Streamer: Intent-Aware CLI Compression for LLMs"
  homepage "https://github.com/mpineda/axiom"
  url "https://github.com/mpineda/axiom/releases/download/v0.1.0/axiom-x86_64-apple-darwin"
  version "0.1.0"
  
  # Note: These SHA256 hashes must be updated per release
  # We recommend using a GitHub Action to update this automatically.
  sha256 "0000000000000000000000000000000000000000000000000000000000000000"

  def install
    # Determine the correct binary name based on the system
    # In a full tap, we would have multiple URLs and SHAs here.
    
    # Install the binary as 'axiom'
    bin.install "axiom-x86_64-apple-darwin" => "axiom"
  end

  def caveats
    <<~EOS
      🚀 Axiom installed successfully!
      
      To complete the industrial setup (Aliases, Shims and AI Context), run:
        axiom install
        
      Restart your terminal after installation.
    EOS
  end

  test do
    system "#{bin}/axiom", "--version"
  end
end
