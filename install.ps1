# Windows Installer for sda-mcp-server
$ErrorActionPreference = "Stop"

$OWNER = "Sudan-Digital-Archive"
$REPO = "mcp-server"
$BIN_NAME = "sda-mcp-server.exe"
$ASSET_SUFFIX = "windows-x86_64"
$ZIP_NAME = "sda-mcp-server-$ASSET_SUFFIX.zip"
$DOWNLOAD_URL = "https://github.com/$OWNER/$REPO/releases/latest/download/$ZIP_NAME"

# Install location: %LOCALAPPDATA%\sda-mcp-server
$INSTALL_DIR = Join-Path $env:LOCALAPPDATA "sda-mcp-server"
$BIN_PATH = Join-Path $INSTALL_DIR $BIN_NAME

Write-Host "Installing sda-mcp-server..." -ForegroundColor Cyan

# Create install directory
if (-not (Test-Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Force -Path $INSTALL_DIR | Out-Null
}

# Download
$TempZip = Join-Path $env:TEMP $ZIP_NAME
Write-Host "Downloading latest release..."
Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $TempZip

# Extract
Write-Host "Extracting..."
Expand-Archive -Path $TempZip -DestinationPath $env:TEMP -Force
$ExtractedBin = Join-Path $env:TEMP $BIN_NAME

# Move to install location
Move-Item -Path $ExtractedBin -Destination $BIN_PATH -Force
Remove-Item $TempZip -Force

Write-Host "Installed to $BIN_PATH" -ForegroundColor Green

# Add to PATH if not present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$INSTALL_DIR*") {
    Write-Host "Adding to User PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$INSTALL_DIR", "User")
    $env:Path += ";$INSTALL_DIR"
    Write-Host "Path updated. You may need to restart your terminal." -ForegroundColor Yellow
} else {
    Write-Host "Already in PATH."
}

Write-Host "✓ Installation complete!" -ForegroundColor Green
Write-Host "Run 'sda-mcp-server --help' to get started."
