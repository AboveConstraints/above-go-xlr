# Above GoXLR - dev environment
# Usage: .\dev.ps1
#
# Closes any running daemon, then starts a fresh one (rebuilt from source via
# `cargo run`, with CORS so the Vite dev server can talk to it) in a separate
# window, then runs the UI dev server with hot-reload.
#
# UI changes (.vue / .js) hot-reload instantly. Rust changes are picked up too,
# because this script kills the old daemon and `cargo run` rebuilds on launch.

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot

Write-Host "`n==> Above GoXLR Dev Environment`n" -ForegroundColor Cyan

# 0. Stop any running daemon/launcher so the fresh build can claim the device
#    and port 14564 (also avoids the "official app is running" preflight clash).
Write-Host "[0/2] Stopping any running GoXLR daemon..." -ForegroundColor Yellow
$stopped = 0
foreach ($name in @("goxlr-daemon", "goxlr-launcher")) {
    Get-Process -Name $name -ErrorAction SilentlyContinue | ForEach-Object {
        try { $_.Kill(); $_.WaitForExit(5000); $stopped++ } catch {}
    }
}
if ($stopped -gt 0) {
    Write-Host "      Stopped $stopped process(es); waiting for port 14564 to free..." -ForegroundColor Green
    for ($i = 0; $i -lt 20; $i++) {
        $busy = Get-NetTCPConnection -LocalPort 14564 -State Listen -ErrorAction SilentlyContinue
        if (-not $busy) { break }
        Start-Sleep -Milliseconds 250
    }
} else {
    Write-Host "      No running daemon found." -ForegroundColor DarkGray
}

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
