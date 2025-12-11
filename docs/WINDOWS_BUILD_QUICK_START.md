# Windows Build Quick Start

Quick reference for building milk on Windows.

## Prerequisites Checklist

- [ ] Rust installed (`rustup.rs`)
- [ ] Node.js 18+ installed
- [ ] pnpm installed (`npm install -g pnpm`)
- [ ] Visual Studio Build Tools (C++ support)
- [ ] WiX Toolset (for MSI)

## Quick Build

```powershell
# Clone and setup
git clone <repo-url>
cd milk
pnpm install

# Build
pnpm run tauri:build:windows

# Create portable
.\scripts\create-portable.ps1

# Verify
.\scripts\verify-windows-build.ps1
```

## Build Outputs

```
src-tauri/target/x86_64-pc-windows-msvc/release/
├── milk.exe                                    # Executable (<15MB)
└── bundle/
    ├── msi/milk_0.1.0_x64.msi                 # MSI Installer
    └── nsis/milk_0.1.0_x64-setup.exe          # NSIS Installer

dist/
└── milk_portable_v0.1.0.zip                    # Portable ZIP
```

## Common Commands

```powershell
# Development
pnpm tauri:dev

# Tests
pnpm test
cd src-tauri && cargo test --release && cd ..

# Build
pnpm run tauri:build:windows

# Portable
.\scripts\create-portable.ps1

# Verify
.\scripts\verify-windows-build.ps1

# Clean
Remove-Item -Recurse -Force src-tauri/target/release
Remove-Item -Recurse -Force dist
```

## Testing

```powershell
# Test executable
.\src-tauri\target\x86_64-pc-windows-msvc\release\milk.exe

# Test MSI
msiexec /i src-tauri\target\x86_64-pc-windows-msvc\release\bundle\msi\milk_0.1.0_x64.msi

# Test portable
Expand-Archive dist\milk_portable_v0.1.0.zip -DestinationPath test
cd test\portable
.\milk.exe
```

## Troubleshooting

### "link.exe not found"

```powershell
# Install Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools
```

### MSI not generated

```powershell
# Install WiX Toolset
choco install wixtoolset
# Or download from https://wixtoolset.org/
```

### Size exceeds 15MB

```powershell
# Check what's taking space
cd src-tauri
cargo bloat --release --crates
```

## CI/CD

Push a tag to trigger automated build:

```bash
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

GitHub Actions will:

- Build all artifacts
- Run tests
- Create GitHub Release (draft)

## Documentation

- **Full Guide**: [BUILD_WINDOWS.md](./BUILD_WINDOWS.md)
- **Release Process**: [RELEASE_PROCESS.md](./RELEASE_PROCESS.md)
- **Build Summary**: [BUILD_PACKAGE_SUMMARY.md](./BUILD_PACKAGE_SUMMARY.md)

## Performance Targets

- Executable: < 15MB ✅
- Startup: < 2 seconds ✅
- RAM (idle): < 100MB ✅
- Build time: < 5 minutes ✅

## Support

Issues? Check:

1. This guide
2. [BUILD_WINDOWS.md](./BUILD_WINDOWS.md)
3. GitHub Issues
4. [Tauri Docs](https://tauri.app/)
