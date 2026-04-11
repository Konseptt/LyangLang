# LyangLang / Lyangpiler — install prebuilt binary for Windows (user PATH).
#
# Usage (PowerShell):
#   irm https://raw.githubusercontent.com/Konseptt/LyangLang/main/install.ps1 | iex
#
# Override repo:  $env:LYANGLANG_REPO = 'owner/LyangLang'; irm ... | iex

$ErrorActionPreference = 'Stop'

$repo = if ($env:LYANGLANG_REPO) { $env:LYANGLANG_REPO } else { 'Konseptt/LyangLang' }
$base = "https://github.com/$repo/releases/latest/download"

Write-Host "Installing Lyangpiler (LyangLang)..." -ForegroundColor Blue

$procArch = [System.Runtime.InteropServices.RuntimeInformation]::ProcessArchitecture
if ($procArch -eq [System.Runtime.InteropServices.Architecture]::Arm64) {
    Write-Host "Prebuilt ARM64 Windows binaries are not published yet." -ForegroundColor Yellow
    Write-Host "Install Rust, then run:" -ForegroundColor Yellow
    Write-Host "  cargo install --git https://github.com/$repo.git --locked" -ForegroundColor Cyan
    exit 1
}
if (-not [Environment]::Is64BitOperatingSystem) {
    Write-Host "32-bit Windows is not supported by prebuilt binaries." -ForegroundColor Red
    Write-Host "Install Rust, then: cargo install --git https://github.com/$repo.git --locked" -ForegroundColor Cyan
    exit 1
}
$arch = 'x86_64'

$asset = "lyangpiler-windows-$arch.zip"
$url = "$base/$asset"
$legacyUrl = "$base/lyangpiler-windows-amd64.zip"

$installRoot = if ($env:LYANGLANG_HOME) { $env:LYANGLANG_HOME } else { Join-Path $env:USERPROFILE '.lyangpiler' }
$binDir = Join-Path $installRoot 'bin'
$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("lyangpiler-install-" + [Guid]::NewGuid().ToString('n'))
$zipPath = Join-Path $tempRoot 'dist.zip'

New-Item -ItemType Directory -Force -Path $tempRoot | Out-Null
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

try {
    Write-Host "Downloading $asset..." -ForegroundColor Blue
    try {
        Invoke-WebRequest -Uri $url -OutFile $zipPath -UseBasicParsing
    } catch {
        if ($arch -eq 'x86_64') {
            Write-Host "Trying legacy asset name..." -ForegroundColor Yellow
            Invoke-WebRequest -Uri $legacyUrl -OutFile $zipPath -UseBasicParsing
        } else {
            throw
        }
    }

    $extractDir = Join-Path $tempRoot 'extract'
    New-Item -ItemType Directory -Force -Path $extractDir | Out-Null
    Expand-Archive -Path $zipPath -DestinationPath $extractDir -Force

    $candidates = @(
        (Join-Path $extractDir 'lyangpiler\lyangpiler.exe'),
        (Join-Path $extractDir 'lyangpiler.exe')
    )
    $exe = $candidates | Where-Object { Test-Path $_ } | Select-Object -First 1
    if (-not $exe) {
        $found = Get-ChildItem -Path $extractDir -Filter lyangpiler.exe -Recurse -ErrorAction SilentlyContinue | Select-Object -First 1
        if ($found) { $exe = $found.FullName }
    }
    if (-not $exe) {
        Write-Host "Could not find lyangpiler.exe inside the zip." -ForegroundColor Red
        Get-ChildItem -Path $extractDir -Recurse -ErrorAction SilentlyContinue | Select-Object FullName
        exit 1
    }

    Copy-Item -Path $exe -Destination (Join-Path $binDir 'lyangpiler.exe') -Force

    $userPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
    if ($userPath -notlike "*$binDir*") {
        [Environment]::SetEnvironmentVariable('PATH', "$userPath;$binDir", 'User')
        Write-Host "Added to user PATH: $binDir" -ForegroundColor Green
    } else {
        Write-Host "PATH already contains $binDir" -ForegroundColor Green
    }

    $machinePath = [Environment]::GetEnvironmentVariable('Path', 'Machine')
    $env:Path = "$machinePath;$([Environment]::GetEnvironmentVariable('Path', 'User'))"

    $installedExe = Join-Path $binDir 'lyangpiler.exe'
    Write-Host ""
    Write-Host "Installed to: $installedExe" -ForegroundColor Green
    Write-Host "PATH updated for this window — you can run lyangpiler now." -ForegroundColor Green
    Write-Host ""
    Write-Host "--- lyangpiler --help ---" -ForegroundColor Gray
    & $installedExe --help
    Write-Host ""
    Write-Host "Try: lyangpiler new demo; cd demo; lyangpiler run main.nbh --vm"
    Write-Host "https://github.com/$repo" -ForegroundColor Blue
}
finally {
    Remove-Item -Path $tempRoot -Recurse -Force -ErrorAction SilentlyContinue
}
