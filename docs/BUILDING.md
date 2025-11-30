# Quick Build Guide

> 📚 **Documentation Map**: See [docs/README.md](README.md) for all documentation  
> 📖 **Detailed Guide**: See [BUILD.md](BUILD.md) for comprehensive build documentation

## Prerequisites

1. **Install Rust**: https://rustup.rs/
2. **Install Node.js 18+**: https://nodejs.org/
3. **Install pnpm**: `npm install -g pnpm`
4. **Install dependencies**: `pnpm install`

## For Windows MSI Installer (Optional)

Install WiX Toolset 3.11+: https://wixtoolset.org/

## Build Commands

### Development Build
```bash
pnpm tauri:dev
```

### Production Build
```bash
pnpm tauri:build
```

This creates:
- `src-tauri/target/release/milk.exe` - Standalone executable
- `src-tauri/target/release/bundle/msi/*.msi` - MSI installer (if WiX installed)
- `src-tauri/target/release/bundle/nsis/*-setup.exe` - NSIS installer (if NSIS installed)

### Create Portable Distribution
```powershell
# Windows PowerShell
.\scripts\create-portable.ps1
```

```bash
# Linux/macOS
./scripts/create-portable.sh
```

### Verify Build
```powershell
# Windows PowerShell
.\scripts\verify-build.ps1
```

## Build Targets

The application meets the following requirements:

- **Executable size**: <15MB (Requirement 8.1)
- **Optimizations**: Size-optimized with LTO enabled
- **File associations**: .wsz and .wal Winamp skins
- **Installers**: MSI and NSIS for Windows

## Troubleshooting

### "linker error" during build
Install Visual Studio Build Tools with C++ support

### MSI not generated
Install WiX Toolset 3.11+ and add to PATH

### Binary too large
Check `Cargo.toml` release profile settings

## More Information

- **[BUILD.md](BUILD.md)** - Detailed build and packaging documentation
- **[BUILD_CONFIGURATION.md](BUILD_CONFIGURATION.md)** - Configuration reference
- **[INSTALLATION_TESTING.md](INSTALLATION_TESTING.md)** - Testing guide
- **[RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md)** - Release process

---

📚 [Back to Documentation Map](README.md)
