# PowerShell script to verify build artifacts and requirements

$ErrorActionPreference = "Stop"

Write-Host "==================================" -ForegroundColor Cyan
Write-Host "milk Build Verification Script" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

$allChecksPassed = $true

# Check 1: Executable exists
Write-Host "1. Checking executable exists..." -ForegroundColor Yellow
$exePath = "src-tauri/target/release/milk.exe"
if (Test-Path $exePath) {
    Write-Host "   ✓ Executable found" -ForegroundColor Green
} else {
    Write-Host "   ✗ Executable not found at $exePath" -ForegroundColor Red
    Write-Host "   Run 'pnpm tauri:build' first" -ForegroundColor Yellow
    $allChecksPassed = $false
}

# Check 2: Executable size (<15MB requirement)
if (Test-Path $exePath) {
    Write-Host "2. Checking executable size..." -ForegroundColor Yellow
    $exeFile = Get-Item $exePath
    $exeSizeMB = [math]::Round($exeFile.Length / 1MB, 2)
    Write-Host "   Size: $exeSizeMB MB" -ForegroundColor Cyan
    
    if ($exeSizeMB -lt 15) {
        Write-Host "   ✓ Size requirement met (<15MB)" -ForegroundColor Green
    } else {
        Write-Host "   ✗ Size exceeds 15MB requirement (Requirement 8.1)" -ForegroundColor Red
        $allChecksPassed = $false
    }
}

# Check 3: MSI installer exists
Write-Host "3. Checking MSI installer..." -ForegroundColor Yellow
$msiPath = Get-ChildItem -Path "src-tauri/target/release/bundle/msi" -Filter "*.msi" -ErrorAction SilentlyContinue | Select-Object -First 1
if ($msiPath) {
    Write-Host "   ✓ MSI installer found: $($msiPath.Name)" -ForegroundColor Green
    $msiSizeMB = [math]::Round($msiPath.Length / 1MB, 2)
    Write-Host "   Size: $msiSizeMB MB" -ForegroundColor Cyan
} else {
    Write-Host "   ⚠ MSI installer not found" -ForegroundColor Yellow
    Write-Host "   This is expected if WiX is not installed" -ForegroundColor Gray
}

# Check 4: NSIS installer exists
Write-Host "4. Checking NSIS installer..." -ForegroundColor Yellow
$nsisPath = Get-ChildItem -Path "src-tauri/target/release/bundle/nsis" -Filter "*-setup.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
if ($nsisPath) {
    Write-Host "   ✓ NSIS installer found: $($nsisPath.Name)" -ForegroundColor Green
    $nsisSizeMB = [math]::Round($nsisPath.Length / 1MB, 2)
    Write-Host "   Size: $nsisSizeMB MB" -ForegroundColor Cyan
} else {
    Write-Host "   ⚠ NSIS installer not found" -ForegroundColor Yellow
    Write-Host "   This is expected if NSIS is not installed" -ForegroundColor Gray
}

# Check 5: Configuration file
Write-Host "5. Checking Tauri configuration..." -ForegroundColor Yellow
$configPath = "src-tauri/tauri.conf.json"
if (Test-Path $configPath) {
    $config = Get-Content $configPath | ConvertFrom-Json
    
    # Check file associations
    if ($config.bundle.windows.fileAssociations) {
        $wsz = $config.bundle.windows.fileAssociations | Where-Object { $_.ext -contains "wsz" }
        $wal = $config.bundle.windows.fileAssociations | Where-Object { $_.ext -contains "wal" }
        
        if ($wsz -and $wal) {
            Write-Host "   ✓ File associations configured (.wsz, .wal)" -ForegroundColor Green
        } else {
            Write-Host "   ✗ File associations incomplete" -ForegroundColor Red
            $allChecksPassed = $false
        }
    } else {
        Write-Host "   ✗ File associations not configured" -ForegroundColor Red
        $allChecksPassed = $false
    }
    
    # Check bundle targets
    if ($config.bundle.targets -contains "msi") {
        Write-Host "   ✓ MSI target configured" -ForegroundColor Green
    } else {
        Write-Host "   ⚠ MSI target not configured" -ForegroundColor Yellow
    }
} else {
    Write-Host "   ✗ Configuration file not found" -ForegroundColor Red
    $allChecksPassed = $false
}

# Check 6: Cargo.toml release profile
Write-Host "6. Checking Cargo release profile..." -ForegroundColor Yellow
$cargoPath = "src-tauri/Cargo.toml"
if (Test-Path $cargoPath) {
    $cargoContent = Get-Content $cargoPath -Raw
    
    $hasOptLevel = $cargoContent -match 'opt-level\s*=\s*"z"'
    $hasLto = $cargoContent -match 'lto\s*=\s*true'
    $hasStrip = $cargoContent -match 'strip\s*=\s*true'
    
    if ($hasOptLevel -and $hasLto -and $hasStrip) {
        Write-Host "   ✓ Release profile optimized for size" -ForegroundColor Green
    } else {
        Write-Host "   ⚠ Release profile not fully optimized" -ForegroundColor Yellow
        if (-not $hasOptLevel) { Write-Host "     Missing: opt-level = 'z'" -ForegroundColor Gray }
        if (-not $hasLto) { Write-Host "     Missing: lto = true" -ForegroundColor Gray }
        if (-not $hasStrip) { Write-Host "     Missing: strip = true" -ForegroundColor Gray }
    }
} else {
    Write-Host "   ✗ Cargo.toml not found" -ForegroundColor Red
    $allChecksPassed = $false
}

# Check 7: Build documentation
Write-Host "7. Checking build documentation..." -ForegroundColor Yellow
if (Test-Path "BUILD.md") {
    Write-Host "   ✓ BUILD.md exists" -ForegroundColor Green
} else {
    Write-Host "   ⚠ BUILD.md not found" -ForegroundColor Yellow
}

# Check 8: Portable distribution scripts
Write-Host "8. Checking portable distribution scripts..." -ForegroundColor Yellow
$hasPortableScript = (Test-Path "scripts/create-portable.ps1") -or (Test-Path "scripts/create-portable.sh")
if ($hasPortableScript) {
    Write-Host "   ✓ Portable distribution scripts exist" -ForegroundColor Green
} else {
    Write-Host "   ⚠ Portable distribution scripts not found" -ForegroundColor Yellow
}

# Summary
Write-Host ""
Write-Host "==================================" -ForegroundColor Cyan
Write-Host "Verification Summary" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan

if ($allChecksPassed) {
    Write-Host "✓ All critical checks passed!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "  1. Test the executable: src-tauri/target/release/milk.exe"
    Write-Host "  2. Test MSI installation (if available)"
    Write-Host "  3. Create portable distribution: .\scripts\create-portable.ps1"
    Write-Host "  4. Test file associations (.wsz, .wal)"
} else {
    Write-Host "✗ Some checks failed. Please review the output above." -ForegroundColor Red
    exit 1
}

Write-Host ""
