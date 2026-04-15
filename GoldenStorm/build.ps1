# ============================================
# GoldenStorm Build System (Final Corrected Edition)
# One‑click build for:
#   - GoldenStorm.exe (UI)
#   - GoldenStormAgent.exe (background agent)
#   - GoldenStormSetup_vX.Y.Z.exe (NSIS installer)
# ============================================

$ErrorActionPreference = "Stop"

function Write-Section($msg) {
    Write-Host ""
    Write-Host "============================================" -ForegroundColor Cyan
    Write-Host " $msg" -ForegroundColor Cyan
    Write-Host "============================================" -ForegroundColor Cyan
}

function Write-Step($msg) {
    Write-Host "→ $msg" -ForegroundColor Yellow
}

function Write-OK($msg) {
    Write-Host "✔ $msg" -ForegroundColor Green
}

function Write-Err($msg) {
    Write-Host "✖ $msg" -ForegroundColor Red
}

Write-Section "GoldenStorm Build System"

# --------------------------------------------
# FIXED: Correct project root for both local + CI
# --------------------------------------------
# build.ps1 lives in GoldenStorm/, repo root is one level up
$projectRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)

# GoldenStorm project folder
$gsProject = Join-Path $projectRoot "GoldenStorm"

# dist folder lives inside GoldenStorm/
$dist = Join-Path $gsProject "dist"

# installer + Cargo.toml inside GoldenStorm/
$installer = Join-Path $gsProject "installer\GoldenStormInstaller.nsi"
$cargoToml = Join-Path $gsProject "Cargo.toml"

# --------------------------------------------
# Extract version from Cargo.toml
# --------------------------------------------
Write-Step "Extracting version from Cargo.toml..."

$versionLine = Select-String -Path $cargoToml -Pattern '^version\s*=\s*".*"$'
if (-not $versionLine) {
    Write-Err "Could not find version in Cargo.toml"
    exit 1
}

$version = ($versionLine.Matches.Value -split '"')[1]
Write-OK "Version detected: $version"

# --------------------------------------------
# Clean old dist folder
# --------------------------------------------
Write-Step "Cleaning old dist folder..."
if (Test-Path $dist) {
    Remove-Item -Recurse -Force $dist
}
New-Item -ItemType Directory -Path $dist | Out-Null
Write-OK "dist folder ready"

# --------------------------------------------
# Build Rust executables
# --------------------------------------------
Write-Step "Building Rust project in release mode..."
cargo build --manifest-path "$gsProject\Cargo.toml" --release
Write-OK "Rust build complete"

# --------------------------------------------
# Verify executables exist
# --------------------------------------------
$exeUI = "$gsProject\target\release\GoldenStorm.exe"
$exeAgent = "$gsProject\target\release\GoldenStormAgent.exe"

if (-not (Test-Path $exeUI)) {
    Write-Err "GoldenStorm.exe missing after build!"
    exit 1
}
if (-not (Test-Path $exeAgent)) {
    Write-Err "GoldenStormAgent.exe missing after build!"
    exit 1
}

Write-OK "Executables verified"

# --------------------------------------------
# Copy executables
# --------------------------------------------
Write-Step "Copying executables to dist..."
Copy-Item $exeUI $dist
Copy-Item $exeAgent $dist
Write-OK "Executables copied"

# --------------------------------------------
# Copy assets
# --------------------------------------------
Write-Step "Copying assets..."
Copy-Item "$gsProject\assets" $dist -Recurse
Write-OK "Assets copied"

# --------------------------------------------
# Stamp version into installer
# --------------------------------------------
Write-Step "Stamping version into NSIS installer script..."

$installerContent = Get-Content $installer
$installerContent = $installerContent -replace '^\s*!define\s+APP_VERSION\s+".*"', "!define APP_VERSION `"$version`""
Set-Content -Path $installer -Value $installerContent

Write-OK "Installer version updated"

# --------------------------------------------
# Build NSIS installer (from inside dist)
# --------------------------------------------
Write-Step "Building NSIS installer..."

Push-Location $dist
makensis "..\installer\GoldenStormInstaller.nsi"
Pop-Location

# --------------------------------------------
# Rename installer to include version
# --------------------------------------------
$installerOut = Join-Path $projectRoot "GoldenStormSetup_v$version.exe"
Move-Item "$projectRoot\GoldenStormSetup.exe" $installerOut -Force

Write-OK "Installer built: GoldenStormSetup_v$version.exe"

# --------------------------------------------
# Final Output Summary
# --------------------------------------------
Write-Section "GoldenStorm Build Complete!"

Write-Host "Output:" -ForegroundColor Green
Write-Host "  ✔ dist\GoldenStorm.exe"
Write-Host "  ✔ dist\GoldenStormAgent.exe"
Write-Host "  ✔ dist\assets\..."
Write-Host "  ✔ GoldenStormSetup_v$version.exe"
Write-Host ""
Write-Host "Build version: $version"
Write-Host "Timestamp: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
Write-Host ""
