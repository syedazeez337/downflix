$ErrorActionPreference = 'Stop'

# Install Node.js LTS via winget if not present
if (-not (Get-Command node -ErrorAction SilentlyContinue)) {
    winget install -e --id OpenJS.NodeJS.LTS
}

# Install Rust via rustup
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    iex ((New-Object System.Net.WebClient).DownloadString('https://sh.rustup.rs')) -ArgumentList '-y'
}

# Refresh path for current session
$env:PATH += ';' + [System.Environment]::GetEnvironmentVariable('USERPROFILE') + '\.cargo\bin'

# Install Tauri CLI
cargo install tauri-cli

# Install yt-dlp if not present
if (-not (Get-Command yt-dlp -ErrorAction SilentlyContinue)) {
    winget install -e --id yt-dlp.yt-dlp
}

# Install Node dependencies
npm install
