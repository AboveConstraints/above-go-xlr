# Above GoXLR - full release build script
# Usage: .\build-release.ps1
# Output: target/release/*.exe (ready to run or package)

param(
    [switch]$SkipUiBuild  # pass -SkipUiBuild if you already built the UI
)

$ErrorActionPreference = "Stop"
$root = $PSScriptRoot

Write-Host "`n==> Above GoXLR Release Build`n" -ForegroundColor Cyan

# 1. Build UI
if (-not $SkipUiBuild) {
    Write-Host "[1/2] Building UI..." -ForegroundColor Yellow
    Push-Location "$root\ui"
    if (-not (Test-Path "node_modules")) {
        npm install
    }
    npm run build
    Pop-Location
    Write-Host "      UI built OK" -ForegroundColor Green
} else {
    Write-Host "[1/2] Skipping UI build (-SkipUiBuild)" -ForegroundColor DarkGray
}

# Copy UI dist → daemon/web-content
Write-Host "      Copying UI to daemon/web-content..." -ForegroundColor Yellow
Copy-Item -Recurse -Force "$root\ui\dist\*" "$root\daemon\web-content\"
Write-Host "      Copied OK" -ForegroundColor Green

# 2. Build Rust (skip build.rs UI step since we already copied)
Write-Host "[2/2] Building Rust (release)..." -ForegroundColor Yellow
cargo build --release
Write-Host "      Rust built OK`n" -ForegroundColor Green

# Summary
$exes = @("goxlr-daemon.exe","goxlr-launcher.exe","goxlr-client.exe","goxlr-defaults.exe")
Write-Host "==> Output files:" -ForegroundColor Cyan
foreach ($exe in $exes) {
    $path = "$root\target\release\$exe"
    if (Test-Path $path) {
        $size = [math]::Round((Get-Item $path).Length / 1MB, 1)
        Write-Host "    $exe  ($size MB)" -ForegroundColor White
    }
}

Write-Host "`n==> To run: .\target\release\goxlr-launcher.exe`n" -ForegroundColor Cyan
