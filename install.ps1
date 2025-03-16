# LyangLang Windows Installer Script
# Usage: iwr -useb https://raw.githubusercontent.com/konseptt/LyangLang/main/install.ps1 | iex

$ErrorActionPreference = 'Stop'

Write-Host "Installing LyangLang..." -ForegroundColor Blue

# Detect architecture
$arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }

# Set installation directory
$installDir = Join-Path $env:USERPROFILE ".lyangpiler"
$binDir = Join-Path $installDir "bin"

# Create installation directories
New-Item -ItemType Directory -Force -Path $installDir | Out-Null
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

# Download latest release
$releaseUrl = "https://github.com/konseptt/LyangLang/releases/latest/download/lyangpiler-windows-$arch.zip"
$zipPath = Join-Path $env:TEMP "lyangpiler.zip"

Write-Host "Downloading LyangLang from $releaseUrl..." -ForegroundColor Blue

try {
    Invoke-WebRequest -Uri $releaseUrl -OutFile $zipPath
    Expand-Archive -Path $zipPath -DestinationPath $installDir -Force
    Remove-Item $zipPath -Force
} catch {
    Write-Host "Failed to download or extract LyangLang: $_" -ForegroundColor Red
    exit 1
}

# Copy executables to bin directory
Copy-Item -Path (Join-Path $installDir "lyangpiler.exe") -Destination $binDir -Force
Copy-Item -Path (Join-Path $installDir "lyangpiler.cmd") -Destination $binDir -Force

# Add to PATH
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if (-not $userPath.Contains($binDir)) {
    [Environment]::SetEnvironmentVariable(
        "PATH",
        "$userPath;$binDir",
        "User"
    )
    Write-Host "Added LyangLang to User PATH" -ForegroundColor Green
}

Write-Host "`nLyangLang has been successfully installed!" -ForegroundColor Green
Write-Host "You can now use" -NoNewline
Write-Host " lyangpiler " -ForegroundColor Blue -NoNewline
Write-Host "to run LyangLang programs."
Write-Host "`nTo get started:"
Write-Host "  lyangpiler new my-project"
Write-Host "  cd my-project"
Write-Host "  lyangpiler run main.nbh --vm"
Write-Host "`nFor more information, visit: https://github.com/konseptt/LyangLang" -ForegroundColor Blue

# Refresh environment variables in the current shell
$env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")