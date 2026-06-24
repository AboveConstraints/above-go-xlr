# Above GoXLR - dev environment
# Usage: .\dev.ps1
#
# Starts the daemon (with CORS so the Vite dev server can talk to it) in a
# separate window, then runs the UI dev server with hot-reload.
#
# UI changes (.vue / .js) hot-reload instantly against your live device — no
# Rust rebuild needed. Only rebuild the daemon if you change Rust code.

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot

Write-Host "`n==> Above GoXLR Dev Environment`n" -ForegroundColor Cyan

# 1. Start the daemon in its own window (CORS enabled for the Vite origin).
#    Uses `cargo run` so Rust changes are picked up on restart.
Write-Host "[1/2] Starting daemon (port 14564, CORS enabled)..." -ForegroundColor Yellow
Start-Process powershell -ArgumentList @(
    "-NoExit", "-Command",
    "Set-Location '$root'; cargo run -p goxlr-daemon -- --http-enable-cors"
)
Write-Host "      Daemon launching in a new window." -ForegroundColor Green

# 2. Run the UI dev server (foreground, hot-reload).
Write-Host "[2/2] Starting UI dev server..." -ForegroundColor Yellow
Push-Location "$root\ui"
if (-not (Test-Path "node_modules")) {
    Write-Host "      Installing UI dependencies (first run)..." -ForegroundColor DarkGray
    npm install
}
Write-Host "`n      UI will open on the Vite URL below (usually http://localhost:5173)" -ForegroundColor Cyan
Write-Host "      It talks to the daemon on http://localhost:14564`n" -ForegroundColor DarkGray
npm run dev
Pop-Location
