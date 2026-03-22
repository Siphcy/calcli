# PowerShell installation script for calcli
$ErrorActionPreference = "Stop"

Write-Host "Fetching latest release..." -ForegroundColor Cyan

try {
    $Release = Invoke-RestMethod -Uri "https://api.github.com/repos/Siphcy/calcli/releases/latest"
    $Version = $Release.tag_name
    $URL = "https://github.com/Siphcy/calcli/releases/download/$Version/calcli-windows-x86_64.exe"

    Write-Host "Downloading calcli $Version..." -ForegroundColor Cyan

    $InstallDir = "$env:USERPROFILE\.local\bin"
    $InstallPath = "$InstallDir\calcli.exe"

    # Create directory if it doesn't exist
    if (-not (Test-Path $InstallDir)) {
        New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    }

    # Download the binary
    Invoke-WebRequest -Uri $URL -OutFile $InstallPath

    # Add to PATH if not already present
    $UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($UserPath -notlike "*$InstallDir*") {
        [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
        Write-Host "`nAdded $InstallDir to PATH" -ForegroundColor Green
        Write-Host "Please restart your terminal for PATH changes to take effect" -ForegroundColor Yellow
    }

    Write-Host "`ncalcli installed successfully to: $InstallPath" -ForegroundColor Green
    Write-Host "`nRun 'calcli' to start (restart terminal first if this is a new installation)" -ForegroundColor Cyan

} catch {
    Write-Host "Error: $_" -ForegroundColor Red
    exit 1
}
