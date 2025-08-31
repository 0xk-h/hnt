#-----------------------------------------------
# HNT Windows Installer
#-----------------------------------------------
# Exit on error
$ErrorActionPreference = "Stop"

#-------------------------------------------------
# Detect architecture
#-------------------------------------------------
$ARCH = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }

#-------------------------------------------------
# Determine installation path
#-------------------------------------------------
$BIN_PATH = "$Env:USERPROFILE\bin"
if (-not (Test-Path $BIN_PATH)) {
    New-Item -ItemType Directory -Path $BIN_PATH | Out-Null
}

#-------------------------------------------------
# Download the binary
#-------------------------------------------------
$BINARY_NAME = "hnt-windows-Hello.exe"
$URL = "https://github.com/kishore399/hnt/releases/latest/download/$BINARY_NAME"
$TEMP_FILE = "$Env:TEMP\hnt.exe"

try {
    Write-Host "Downloading $BINARY_NAME from $URL ..."
    Invoke-WebRequest -Uri $URL -OutFile $TEMP_FILE -UseBasicParsing
} catch {
    Write-Host "❌ The binary for your OS/architecture is not published yet."
    Write-Host "Please download the source manually from GitHub and compile it:"
    Write-Host "https://github.com/kishore399/hnt"
    exit 1
}

#-------------------------------------------------
# Move binary to installation path
#-------------------------------------------------
Move-Item -Path $TEMP_FILE -Destination "$BIN_PATH\hnt.exe" -Force

Write-Host "✅ Installation complete! Run 'hnt guess' to verify."
Write-Host "Installed at: $BIN_PATH\hnt.exe"

# Adding $BIN_PATH to user PATH
$PATH_ENTRY = [Environment]::GetEnvironmentVariable("Path", "User")
if ($PATH_ENTRY -notlike "*$BIN_PATH*") {
    [Environment]::SetEnvironmentVariable("Path", "$PATH_ENTRY;$BIN_PATH", "User")
    Write-Host "Added $BIN_PATH to user PATH. Restart PowerShell to use 'hnt' globally."
}
for this