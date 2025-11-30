# Installation Testing Guide

This guide describes how to test the milk installation packages.

> 📚 **Documentation Map**: See [docs/README.md](README.md) for all documentation  
> 📖 **Build Guide**: See [BUILD.md](BUILD.md) for build instructions  
> ✅ **Release Checklist**: See [RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md) for release process

## Prerequisites

- Clean Windows 10/11 system (or VM)
- Test .wsz and .wal skin files
- Test audio files (mp3, flac, wav)

## MSI Installer Testing

### Installation Test

1. **Download/Locate MSI**
   - File: `milk_0.1.0_x64_en-US.msi`
   - Location: `src-tauri/target/release/bundle/msi/`

2. **Install Application**
   ```powershell
   # Interactive installation
   .\milk_0.1.0_x64_en-US.msi
   
   # Silent installation (for testing)
   msiexec /i milk_0.1.0_x64_en-US.msi /quiet
   ```

3. **Verify Installation**
   - [ ] Application installed to Program Files
   - [ ] Start Menu shortcut created
   - [ ] Desktop shortcut created (if selected)
   - [ ] Application appears in "Add or Remove Programs"

4. **Launch Application**
   - [ ] Click Start Menu shortcut
   - [ ] Application launches within 2 seconds
   - [ ] farmer appears and prompts for setup

5. **First Run Setup**
   - [ ] farmer prompts for music library path
   - [ ] Select a folder with audio files
   - [ ] Library scanning completes successfully
   - [ ] farmer transitions to idle state

### File Association Test

1. **Download Test Skins**
   - Get a .wsz file (e.g., from Winamp skin archive)
   - Get a .wal file

2. **Test .wsz Association**
   - [ ] Double-click .wsz file
   - [ ] milk launches (or comes to foreground)
   - [ ] Skin loads successfully
   - [ ] UI updates with new skin

3. **Test .wal Association**
   - [ ] Double-click .wal file
   - [ ] milk launches (or comes to foreground)
   - [ ] Skin loads successfully
   - [ ] UI updates with new skin

4. **Verify Default Programs**
   - [ ] Open Windows Settings → Apps → Default apps
   - [ ] Search for .wsz
   - [ ] Verify milk is listed as an option
   - [ ] Search for .wal
   - [ ] Verify milk is listed as an option

### Uninstallation Test

1. **Uninstall via Settings**
   - Open Windows Settings → Apps → Installed apps
   - Find "milk"
   - Click Uninstall
   - [ ] Uninstaller runs successfully
   - [ ] Application removed from Program Files
   - [ ] Start Menu shortcuts removed
   - [ ] Desktop shortcut removed (if created)

2. **Verify User Data Preserved**
   - [ ] Check `%APPDATA%\com.milk.player\` still exists
   - [ ] Configuration file preserved
   - [ ] Playlists preserved
   - [ ] Logs preserved

3. **Reinstall Test**
   - Reinstall the application
   - [ ] Previous configuration loaded
   - [ ] Previous playlists available
   - [ ] No setup wizard (already configured)

## NSIS Installer Testing

### Installation Test

1. **Download/Locate NSIS Installer**
   - File: `milk_0.1.0_x64-setup.exe`
   - Location: `src-tauri/target/release/bundle/nsis/`

2. **Install Application**
   - Run the installer
   - [ ] Modern installer UI appears
   - [ ] License agreement displayed (if configured)
   - [ ] Installation directory selectable
   - [ ] Installation completes successfully

3. **Verify Installation**
   - [ ] Application installed to selected directory
   - [ ] Start Menu shortcut created
   - [ ] Uninstaller created

4. **Launch and Test**
   - Same as MSI testing above

### Uninstallation Test

1. **Uninstall via Uninstaller**
   - Run uninstaller from Start Menu or installation directory
   - [ ] Uninstaller runs successfully
   - [ ] Application files removed
   - [ ] Start Menu shortcuts removed

2. **Verify User Data**
   - Same as MSI testing above

## Portable Version Testing

### Setup Test

1. **Extract Portable ZIP**
   - File: `milk_portable_v0.1.0.zip`
   - Extract to a test folder (e.g., `C:\PortableApps\milk\`)

2. **Verify Contents**
   - [ ] milk.exe present
   - [ ] README.txt present
   - [ ] assets/ folder present

3. **Run Application**
   - [ ] Double-click milk.exe
   - [ ] Application launches without installation
   - [ ] No admin rights required
   - [ ] farmer prompts for setup

### Configuration Test

1. **Complete Setup**
   - Configure library path
   - Create a playlist
   - Change settings (volume, skin)

2. **Verify Local Storage**
   - [ ] Check for config file in application directory
   - [ ] Configuration persists after restart
   - [ ] Playlists saved locally

3. **Portability Test**
   - [ ] Copy entire folder to different location
   - [ ] Run milk.exe from new location
   - [ ] Configuration and playlists still available

### No Installation Pollution Test

1. **Check System**
   - [ ] No entries in Program Files
   - [ ] No Start Menu shortcuts
   - [ ] No registry entries
   - [ ] No entries in Add/Remove Programs

2. **Clean Removal**
   - [ ] Delete portable folder
   - [ ] System returns to clean state

## Performance Testing

### Startup Time Test

1. **Measure Cold Start**
   ```powershell
   Measure-Command { Start-Process "milk.exe" }
   ```
   - [ ] Startup time <2 seconds (Requirement 8.3)

2. **Measure Warm Start**
   - Close application
   - Launch again immediately
   - [ ] Startup time <2 seconds

### Memory Usage Test

1. **Idle Memory**
   - Launch application
   - Wait for idle state
   - Check Task Manager
   - [ ] RAM usage <100MB (Requirement 8.2)

2. **Playback Memory**
   - Play an audio file
   - Check Task Manager
   - [ ] RAM usage reasonable (<150MB)

3. **Visualizer Memory**
   - Enable visualizer
   - Check Task Manager
   - [ ] RAM usage reasonable (<200MB)

### Binary Size Test

```powershell
(Get-Item "milk.exe").Length / 1MB
```
- [ ] Executable size <15MB (Requirement 8.1)

## Functional Testing

### Audio Playback

1. **Local Files**
   - [ ] MP3 playback works
   - [ ] FLAC playback works
   - [ ] WAV playback works
   - [ ] Metadata displays correctly
   - [ ] Album art displays (if embedded)

2. **Playback Controls**
   - [ ] Play button works
   - [ ] Pause button works
   - [ ] Stop button works
   - [ ] Next track works
   - [ ] Previous track works
   - [ ] Seek bar works
   - [ ] Volume control works

### Playlist Management

1. **Create Playlist**
   - [ ] Create new playlist
   - [ ] Add tracks to playlist
   - [ ] Reorder tracks
   - [ ] Remove tracks
   - [ ] Save playlist

2. **Load Playlist**
   - [ ] Load saved playlist
   - [ ] Play from playlist
   - [ ] Queue updates correctly

### Visualizer

1. **Activation**
   - [ ] Visualizer displays during playback
   - [ ] Visualizer stops when playback stops
   - [ ] Frame rate 30+ FPS

2. **Style Switching**
   - [ ] Switch to bars style
   - [ ] Switch to waveform style
   - [ ] Switch to spectrum style
   - [ ] No playback interruption

### farmer Buddy

1. **States**
   - [ ] Idle state animations work
   - [ ] Listening state during playback
   - [ ] Prompting state during setup
   - [ ] Error state on invalid input
   - [ ] Celebrating state on completion

2. **Interactions**
   - [ ] Speech bubbles display correctly
   - [ ] Animations smooth
   - [ ] Reactions to playback events

### Skin System

1. **Default Skin**
   - [ ] Default skin loads on first run
   - [ ] UI elements visible and functional

2. **Custom Skins**
   - [ ] Load .wsz skin from menu
   - [ ] Load .wal skin from menu
   - [ ] Skin applies correctly
   - [ ] Skin persists after restart

3. **Error Handling**
   - [ ] Corrupted skin falls back to default
   - [ ] farmer shows error message
   - [ ] Application remains functional

## Streaming Integration Testing

### Spotify (if configured)

1. **Authentication**
   - [ ] OAuth flow works
   - [ ] Credentials stored securely
   - [ ] Token refresh works

2. **Metadata Sync**
   - [ ] "Now Playing" displays
   - [ ] Track changes update <2 seconds
   - [ ] Metadata complete (title, artist, album)

### YouTube (if configured)

1. **Authentication**
   - [ ] API key validation works
   - [ ] Credentials stored securely

2. **Metadata Sync**
   - [ ] Video info displays
   - [ ] Updates <2 seconds
   - [ ] Metadata complete

## Error Handling Testing

### Invalid Input

1. **Setup Errors**
   - [ ] Invalid library path shows error
   - [ ] farmer displays error state
   - [ ] User can retry

2. **File Errors**
   - [ ] Corrupted audio file handled gracefully
   - [ ] Missing file handled gracefully
   - [ ] Error message displayed

### Configuration Errors

1. **Corrupted Config**
   - Delete or corrupt config file
   - [ ] Application creates default config
   - [ ] farmer notifies user
   - [ ] Application remains functional

2. **Missing Permissions**
   - [ ] Read-only directory handled
   - [ ] Error message displayed
   - [ ] Application remains functional

## Regression Testing

After any updates:

1. **Upgrade Test**
   - Install old version
   - Configure application
   - Install new version over old
   - [ ] Configuration preserved
   - [ ] Playlists preserved
   - [ ] Application works correctly

2. **Downgrade Test**
   - Install new version
   - Configure application
   - Uninstall and install old version
   - [ ] Configuration compatible
   - [ ] Application works correctly

## Test Report Template

```
Test Date: ___________
Tester: ___________
Version: ___________
OS: Windows ___ (Build: _______)

Installation Type: [ ] MSI  [ ] NSIS  [ ] Portable

Results:
- Installation: [ ] Pass  [ ] Fail
- File Associations: [ ] Pass  [ ] Fail
- Performance: [ ] Pass  [ ] Fail
- Functionality: [ ] Pass  [ ] Fail
- Uninstallation: [ ] Pass  [ ] Fail

Issues Found:
1. ___________
2. ___________

Notes:
___________
```

## Automated Testing

For CI/CD integration, see `.github/workflows/build.yml.template`

## Related Documentation

- **[BUILD.md](BUILD.md)** - Build and packaging guide
- **[RELEASE_CHECKLIST.md](RELEASE_CHECKLIST.md)** - Release process checklist
- **[BUILD_CONFIGURATION.md](BUILD_CONFIGURATION.md)** - Configuration reference
- **[PERFORMANCE_OPTIMIZATIONS.md](PERFORMANCE_OPTIMIZATIONS.md)** - Performance testing

---

📚 [Back to Documentation Map](README.md)
