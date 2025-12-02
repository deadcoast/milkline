# PowerShell script to verify Windows build meets all requirements

$ErrorActionPreference = "Stop"

Write-Host "==================================" -ForegroundColor Cyan
Write-Host "milk Windows Build Verification" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan
Write-Host ""

$allChecksPassed = $true

# Check 1: Verify executable exists
Write-Host "1. Checking executable exists..." -ForegroundColor Yellow
$exePath = "src-tauri/target/x86_64-pc-windows-msvc/release/milk.exe"
if (Test-Path $exePath) {
    Write-Host "   ✓ Executable found" -ForegroundColor Green
} else {
    Write-Host "   ✗ Executable not found at: $exePath" -ForegroundColor Red
    Write-Host "   Run: pnpm run tauri:build:windows" -ForegroundColor Yellow
    $allChecksPassed = $false
}

# Check 2: Verify executable size
if (Test-Path $exePath) {
    Write-Host "2. Checking executable size..." -ForegroundColor Yellow
    $exe = Get-Item $exePath
    $sizeMB = [math]::Round($exe.Length / 1MB, 2)
    Write-Host "   Size: $sizeMB MB" -ForegroundColor Cyan
    
    if ($sizeMB -lt 15) {
        Write-Host "   ✓ Size requirement met (<15MB)" -ForegroundColor Green
    } else {
        Write-Host "   ⚠ Warning: Exceeds 15MB target" -ForegroundColor Yellow
        Write-Host "   Consider optimizing dependencies or build settings" -ForegroundColor Yellow
    }
}

# Check 3: Verify MSI installer exists
Write-Host "3. Checking MSI installer..." -ForegroundColor Yellow
$msiPath = "src-tauri/target/x86_64-pc-windows-msvc/release/bundle/msi/milk_0.1.0_x64.msi"
if (Test-Path $msiPath) {
    Write-Host "   ✓ MSI installer found" -ForegroundColor Green
    $msi = Get-Item $msiPath
    $msiSizeMB = [math]::Round($msi.Length / 1MB, 2)
    Write-Host "   MSI Size: $msiSizeMB MB" -ForegroundColor Cyan
} else {
    Write-Host "   ✗ MSI installer not found" -ForegroundColor Red
    Write-Host "   Ensure WiX Toolset is installed" -ForegroundColor Yellow
    $allChecksPassed = $false
}

# Check 4: Verify NSIS installer exists
Write-Host "4. Checking NSIS installer..." -ForegroundColor Yellow
$nsisPath = "src-tauri/target/x86_64-pc-windows-msvc/release/bundle/nsis/milk_0.1.0_x64-setup.exe"
if (Test-Path $nsisPath) {
    Write-Host "   ✓ NSIS installer found" -ForegroundColor Green
    $nsis = Get-Item $nsisPath
    $nsisSizeMB = [math]::Round($nsis.Length / 1MB, 2)
    Write-Host "   NSIS Size: $nsisSizeMB MB" -ForegroundColor Cyan
} else {
    Write-Host "   ⚠ NSIS installer not found (optional)" -ForegroundColor Yellow
}

# Check 5: Verify file association configuration
Write-Host "5. Checking file association configuration..." -ForegroundColor Yellow
$wixPath = "src-tauri/wix/file-associations.wxs"
if (Test-Path $wixPath) {
    Write-Host "   ✓ File association config found" -ForegroundColor Green
    
    # Check for .wsz and .wal associations
    $wixContent = Get-Content $wixPath -Raw
    if ($wixContent -match '\.wsz' -and $wixContent -match '\.wal') {
        Write-Host "   ✓ .wsz and .wal associations configured" -ForegroundColor Green
    } else {
        Write-Host "   ⚠ Missing .wsz or .wal associations" -ForegroundColor Yellow
    }
} else {
    Write-Host "   ✗ File association config not found" -ForegroundColor Red
    $allChecksPassed = $false
}

# Check 6: Verify icons exist
Write-Host "6. Checking application icons..." -ForegroundColor Yellow
$iconFiles = @(
    "src-tauri/icons/icon.ico",
    "src-tauri/icons/icon.png",
    "src-tauri/icons/32x32.png",
    "src-tauri/icons/128x128.png"
)
$allIconsExist = $true
foreach ($icon in $iconFiles) {
    if (-not (Test-Path $icon)) {
        Write-Host "   ✗ Missing: $icon" -ForegroundColor Red
        $allIconsExist = $false
    }
}
if ($allIconsExist) {
    Write-Host "   ✓ All required icons present" -ForegroundColor Green
} else {
    $allChecksPassed = $false
}

# Check 7: Verify Tauri configuration
Write-Host "7. Checking Tauri configuration..." -ForegroundColor Yellow
$tauriConfig = "src-tauri/tauri.conf.json"
if (Test-Path $tauriConfig) {
    $config = Get-Content $tauriConfig | ConvertFrom-Json
    
    # Check bundle targets
    if ($config.bundle.targets -contains "msi") {
        Write-Host "   ✓ MSI target configured" -ForegroundColor Green
    } else {
        Write-Host "   ⚠ MSI target not configured" -ForegroundColor Yellow
    }
    
    # Check product name
    if ($config.productName -eq "milk") {
        Write-Host "   ✓ Product name correct" -ForegroundColor Green
    } else {
        Write-Host "   ⚠ Product name: $($config.productName)" -ForegroundColor Yellow
    }
    
    # Check identifier
    if ($config.identifier) {
        Write-Host "   ✓ App identifier: $($config.identifier)" -ForegroundColor Green
    }
} else {
    Write-Host "   ✗ Tauri config not found" -ForegroundColor Red
    $allChecksPassed = $false
}

# Check 8: Verify Cargo release profile
Write-Host "8. Checking Cargo release profile..." -ForegroundColor Yellow
$cargoToml = "src-tauri/Cargo.toml"
if (Test-Path $cargoToml) {
    $cargoContent = Get-Content $cargoToml -Raw
    
    $optimizations = @{
        'opt-level = "z"' = "Size optimization"
        'lto = true' = "Link-time optimization"
        'strip = true' = "Symbol stripping"
        'panic = "abort"' = "Panic abort"
    }
    
    $allOptimized = $true
    foreach ($opt in $optimizations.Keys) {
        if ($cargoContent -match [regex]::Escape($opt)) {
            Write-Host "   ✓ $($optimizations[$opt])" -ForegroundColor Green
        } else {
            Write-Host "   ⚠ Missing: $($optimizations[$opt])" -ForegroundColor Yellow
            $allOptimized = $false
        }
    }
} else {
    Write-Host "   ✗ Cargo.toml not found" -ForegroundColor Red
    $allChecksPassed = $false
}

# Check 9: Test executable (if it exists)
if (Test-Path $exePath) {
    Write-Host "9. Testing executable..." -ForegroundColor Yellow
    
    # Check if it's a valid PE file
    try {
        $peHeader = [System.IO.File]::ReadAllBytes($exePath)[0..1]
        if ($peHeader[0] -eq 0x4D -and $peHeader[1] -eq 0x5A) {
            Write-Host "   ✓ Valid Windows executable" -ForegroundColor Green
        } else {
            Write-Host "   ✗ Invalid executable format" -ForegroundColor Red
            $allChecksPassed = $false
        }
    } catch {
        Write-Host "   ⚠ Could not verify executable format" -ForegroundColor Yellow
    }
    
    # Check dependencies (optional)
    Write-Host "   Checking dependencies..." -ForegroundColor Cyan
    try {
        $dumpbin = Get-Command dumpbin -ErrorAction SilentlyContinue
        if ($dumpbin) {
            $deps = & dumpbin /dependents $exePath 2>&1 | Select-String "\.dll"
            Write-Host "   Dependencies found: $($deps.Count)" -ForegroundColor Cyan
        }
    } catch {
        # dumpbin not available, skip
    }
}

# Check 10: Verify portable distribution script
Write-Host "10. Checking portable distribution script..." -ForegroundColor Yellow
$portableScript = "scripts/create-portable.ps1"
if (Test-Path $portableScript) {
    Write-Host "   ✓ Portable script found" -ForegroundColor Green
} else {
    Write-Host "   ✗ Portable script not found" -ForegroundColor Red
    $allChecksPassed = $false
}

# Summary
Write-Host ""
Write-Host "==================================" -ForegroundColor Cyan
Write-Host "Verification Summary" -ForegroundColor Cyan
Write-Host "==================================" -ForegroundColor Cyan

if ($allChecksPassed) {
    Write-Host "✓ All checks passed!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Cyan
    Write-Host "1. Test the executable: .\$exePath" -ForegroundColor White
    Write-Host "2. Test MSI installation" -ForegroundColor White
    Write-Host "3. Create portable distribution: .\scripts\create-portable.ps1" -ForegroundColor White
    Write-Host "4. Test file associations (.wsz, .wal)" -ForegroundColor White
    exit 0
} else {
    Write-Host "✗ Some checks failed" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please review the errors above and:" -ForegroundColor Yellow
    Write-Host "1. Ensure all prerequisites are installed" -ForegroundColor White
    Write-Host "2. Run: pnpm run tauri:build:windows" -ForegroundColor White
    Write-Host "3. Re-run this verification script" -ForegroundColor White
    exit 1
}
