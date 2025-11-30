# Release Checklist

Use this checklist before creating a new release of milk.

## Pre-Release

### Version Updates
- [ ] Update version in `package.json`
- [ ] Update version in `src-tauri/Cargo.toml`
- [ ] Update version in `src-tauri/tauri.conf.json`
- [ ] Update CHANGELOG.md with release notes

### Code Quality
- [ ] All tests pass: `pnpm test`
- [ ] No TypeScript errors: `pnpm check`
- [ ] Rust code compiles without warnings: `cd src-tauri && cargo clippy`
- [ ] Code formatted: `cd src-tauri && cargo fmt`

### Build Verification
- [ ] Clean build succeeds: `pnpm tauri:build`
- [ ] Executable size <15MB (Requirement 8.1)
- [ ] Run verification script: `.\scripts\verify-build.ps1`

## Build Artifacts

### Executable
- [ ] `milk.exe` builds successfully
- [ ] Executable runs without errors
- [ ] Startup time <2 seconds (Requirement 8.3)
- [ ] Memory usage <100MB idle (Requirement 8.2)

### Installers
- [ ] MSI installer generated (if WiX installed)
- [ ] NSIS installer generated (if NSIS installed)
- [ ] Installers include proper metadata
- [ ] File associations configured (.wsz, .wal)

### Portable Distribution
- [ ] Create portable ZIP: `.\scripts\create-portable.ps1`
- [ ] Portable version includes README
- [ ] Portable version runs without installation

## Testing

### Installation Testing
- [ ] Install via MSI on clean Windows system
- [ ] Verify Start Menu shortcuts created
- [ ] Verify file associations work (.wsz, .wal)
- [ ] Application launches successfully
- [ ] First-run setup flow works

### Functional Testing
- [ ] Local audio playback works (mp3, flac, wav)
- [ ] Library scanning works
- [ ] Playlist creation and management works
- [ ] Volume control works
- [ ] Visualizer displays correctly
- [ ] farmer animations work
- [ ] Configuration persists across restarts

### Skin Testing
- [ ] Default skin loads
- [ ] Custom .wsz skin loads
- [ ] Custom .wal skin loads
- [ ] Skin fallback works for corrupted files
- [ ] Double-clicking .wsz file opens in milk

### Streaming Integration (if configured)
- [ ] Spotify authentication works
- [ ] Spotify metadata sync works
- [ ] YouTube authentication works
- [ ] YouTube metadata sync works

### Uninstallation Testing
- [ ] Uninstall via Windows Settings
- [ ] All program files removed
- [ ] Start Menu shortcuts removed
- [ ] User data preserved in AppData
- [ ] Registry entries cleaned up

### Portable Testing
- [ ] Extract portable ZIP
- [ ] Run milk.exe without installation
- [ ] Configuration stored locally
- [ ] Application works without admin rights

## Performance Verification

- [ ] Startup time: <2 seconds (Requirement 8.3)
- [ ] RAM usage (idle): <100MB (Requirement 8.2)
- [ ] Executable size: <15MB (Requirement 8.1)
- [ ] Visualizer FPS: 30+ (Requirement 5.3)
- [ ] Metadata sync: <2 seconds (Requirements 2.3, 3.3)

## Documentation

- [ ] README.md updated
- [ ] BUILD.md reviewed and accurate
- [ ] BUILDING.md quick guide accurate
- [ ] CHANGELOG.md includes all changes
- [ ] Release notes prepared

## Git and GitHub

- [ ] All changes committed
- [ ] Version bump committed: `git commit -am "Release vX.Y.Z"`
- [ ] Tag created: `git tag vX.Y.Z`
- [ ] Changes pushed: `git push && git push --tags`

## GitHub Release

- [ ] Create new release on GitHub
- [ ] Upload `milk.exe`
- [ ] Upload MSI installer (if available)
- [ ] Upload NSIS installer (if available)
- [ ] Upload portable ZIP
- [ ] Include release notes
- [ ] Mark as pre-release if applicable

## Post-Release

- [ ] Verify download links work
- [ ] Test installation from GitHub release
- [ ] Update project website (if applicable)
- [ ] Announce release (if applicable)
- [ ] Monitor for issues

## Notes

- User data location: `%APPDATA%\com.milk.player\`
- Configuration file: `%APPDATA%\com.milk.player\config.json`
- Logs location: `%APPDATA%\com.milk.player\logs\`
- Playlists location: `%APPDATA%\com.milk.player\playlists\`

## Rollback Plan

If critical issues are discovered:
1. Mark release as pre-release on GitHub
2. Add warning to release notes
3. Prepare hotfix
4. Follow this checklist for hotfix release
