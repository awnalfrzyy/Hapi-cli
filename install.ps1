# PowerShell installer for Windows
$ErrorActionPreference = 'Stop'

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo not found. Install Rust from https://rustup.rs/ first."
    exit 1
}

if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Error "git not found. Please install git first."
    exit 1
}

Write-Host "==> Updating dependencies and building release"
cargo update
cargo build --release

$binPath = Join-Path (Resolve-Path .).Path 'target\release\http-req.exe'
if (-not (Test-Path $binPath)) {
    Write-Error "Build failed or binary not found: $binPath"
    exit 1
}

Write-Host "==> Installation complete. Binary: $binPath"
Write-Host "You can add this to PATH or run directly."
Write-Host "Example (Admin): [Environment]::SetEnvironmentVariable('Path', $env:Path + ';' + (Split-Path $binPath), 'Machine')"
