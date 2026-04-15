# ============================================
# GoldenStorm Build System (TRUE FINAL EDITION)
# ============================================

$ErrorActionPreference = "Stop"

function Write-Section($msg) {
    Write-Host ""
    Write-Host "============================================" -ForegroundColor Cyan
    Write-Host " $msg" -ForegroundColor Cyan
    Write-Host "============================================" -ForegroundColor Cyan
}

function Write-Step($msg) { Write-Host "→ $msg" -ForegroundColor Yellow }
function Write-OK($msg)  { Write-Host "✔ $msg" -ForegroundColor Green }
function Write-Err($msg) { Write-Host "✖ $msg" -ForegroundColor Red }

Write-Section "GoldenStorm Build System"

# --------------------------------------------
# Correct repo root + project folder
# --------------------------------------------
$projectRoot = Split-Path -Parent (Split-Path -Parent $MyInvocation.MyCommand.Path)
$gsProject   = Join-Path $projectRoot "GoldenStorm"

$dist       = Join-Path $gsProject "dist"
$installer  = Join-Path $gsProject "installer\GoldenStormInstaller.nsi"
$cargoToml  = Join-Path $gsProject "Cargo.toml"

# --------------------------------------------
# Extract version
# --------------------------------------------
Write-Step "Extracting version from Cargo.toml..."
$versionLine = Select-String -Path $cargoToml -Pattern '^version\s*=\s*".*"$'
if (-not $versionLine) { Write-Err "Version not found"; exit 1 }
$version = ($versionLine.Matches.Value -split '"')[1]
Write-OK "Version detected: $version"

# --------------------------------------------
# Clean dist
# --------------------------------------------
Write-Step "Cleaning dist folder..."
if (Test-Path $dist) { Remove-Item -Recurse -Force $dist }
New-Item -ItemType Directory -Path $dist | Out-Null
Write-OK "dist ready"

# --------------------------------------------
# Build Rust
# --------------------------------------------
Write-Step "Building Rust project..."
cargo build --manifest-path "$gsProject\Cargo.toml" --release
Write-OK "Rust build complete"

# --------------------------------------------
# Verify binaries (REAL PATH)
# --------------------------------------------
$exeUI    = "$gsProject\target\release\GoldenStorm.exe"
$exeAgent = "$gsProject\target\release\GoldenStormAgent.exe"

if (-not (Test-Path $exeUI))    { Write-Err "GoldenStorm.exe missing"; exit 1 }
if (-not (Test-Path $exeAgent)) { Write-Err "GoldenStormAgent.exe missing"; exit 1 }

Write-OK "Executables verified"

# --------------------------------------------
# Copy binaries
# --------------------------------------------
Write-Step "Copying executables..."
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
# Update installer version
# --------------------------------------------
Write-Step "Stamping version into installer..."
$installerContent = Get-Content $installer
$installerContent = $installerContent -replace '^\s*!define\s+APP_VERSION\s+".*"', "!define APP_VERSION `"$version`""
Set-Content -Path $installer -Value $installerContent
Write-OK "Installer version updated"

# --------------------------------------------
# Build NSIS installer (RUN FROM DIST)
# --------------------------------------------
Write-Step "Building NSIS installer..."
Push-Location $dist
makensis "..\installer\GoldenStormInstaller.nsi"
Pop-Location

# --------------------------------------------
# Move installer (REAL LOCATION IS dist)
# --------------------------------------------
$installerOut = Join-Path $projectRoot "GoldenStormSetup_v$version.exe"
Move-Item "$dist\GoldenStormSetup.exe" $installerOut -Force

Write-OK "Installer built: GoldenStormSetup_v$version.exe"

# --------------------------------------------
# Summary
# --------------------------------------------
Write-Section "GoldenStorm Build Complete!"
Write-Host "✔ dist\GoldenStorm.exe"
Write-Host "✔ dist\GoldenStormAgent.exe"
Write-Host "✔ dist\assets\..."
Write-Host "✔ GoldenStormSetup_v$version.exe"
Write-Host ""
Write-Host "Build version: $version"
Write-Host "Timestamp: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
Write-Host ""
