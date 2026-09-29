# Builds the codegraph binary (release) and copies it into plugin/bin/.
# Usage: powershell -ExecutionPolicy Bypass -File scripts/build.ps1
$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
$cargoBin = Join-Path $env:USERPROFILE '.cargo\bin'
if (-not (Get-Command cargo -ErrorAction SilentlyContinue) -and (Test-Path $cargoBin)) {
    $env:Path = "$cargoBin;$env:Path"
}
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw 'cargo not found: install Rust with rustup (https://rustup.rs)'
}

Push-Location $root
try {
    cargo build --release -p codegraph
    if ($LASTEXITCODE -ne 0) { throw "cargo build failed with exit code $LASTEXITCODE" }
} finally {
    Pop-Location
}

$source = Join-Path $root 'target\release\codegraph.exe'
$binDir = Join-Path $root 'plugin\bin'
$target = Join-Path $binDir 'codegraph.exe'
New-Item -ItemType Directory -Force -Path $binDir | Out-Null

# A running `codegraph mcp` (an open Claude Code session) locks the exe. Windows
# allows renaming a running exe, so move it aside and copy the new one in.
$old = "$target.old"
if (Test-Path $old) {
    try { Remove-Item -Force $old } catch { Write-Warning "cannot remove $old (still running?)" }
}
if (Test-Path $target) {
    try {
        Copy-Item -Force $source $target
    } catch {
        if (Test-Path $old) { throw "cannot replace $target and $old is still in use: close Claude Code sessions" }
        Rename-Item -Path $target -NewName (Split-Path -Leaf $old)
        Copy-Item -Force $source $target
        Write-Warning "$target was in use: the old binary is kept as $old until sessions restart"
    }
} else {
    Copy-Item -Force $source $target
}

& $target --version
if ($LASTEXITCODE -ne 0) { throw "$target --version failed" }
Write-Host "plugin binary: $target"
