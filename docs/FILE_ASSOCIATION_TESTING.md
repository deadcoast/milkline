# File Association Testing Guide

This document describes how to test the file association handling for Winamp skin files (.wsz and .wal).

## Features Implemented

1. **WiX File Associations**: Configured in `src-tauri/wix/file-associations.wxs`
   - Associates .wsz files with milk
   - Associates .wal files with milk
   - Registers "Open with milk" context menu option

2. **Command-Line Argument Handling**: Implemented in `src-tauri/src/lib.rs`
   - Detects when the application is launched with a skin file path
   - Emits a `load-skin-file` event to the frontend
   - Logs file association events for debugging

3. **Drag-and-Drop Support**: Implemented in `src/routes/+page.svelte`
   - Accepts .wsz and .wal files dropped onto the application window
   - Shows visual feedback during drag operation
   - Loads and applies the skin automatically
   - Updates farmer buddy state during skin loading

## Testing Instructions

### 1. Testing File Associations (Windows Only)

**Prerequisites:**

- Build and install the application using the MSI installer
- Have a .wsz or .wal skin file available for testing

**Steps:**

1. Build the MSI installer:

   ```bash
   pnpm tauri build
   ```

2. Install the application using the generated MSI file

3. Navigate to a .wsz or .wal file in File Explorer

4. Right-click the file and select "Open with milk"

5. The application should launch and automatically load the selected skin

**Expected Behavior:**

- Application launches
- Farmer buddy shows "Loading new skin..." message
- Skin is applied to the interface
- Farmer buddy shows "Skin loaded successfully!" and celebrates
- Configuration is updated with the new skin path

### 2. Testing Drag-and-Drop

**Prerequisites:**

- Application is running
- Have a .wsz or .wal skin file available

**Steps:**

1. Launch the milk application

2. Open File Explorer and navigate to a skin file

3. Drag the skin file over the application window

4. Observe the visual feedback (dashed border and drop overlay)

5. Drop the file onto the application

**Expected Behavior:**

- While dragging: Dashed border appears around the window
- While dragging: Drop overlay shows "Drop skin file here" message
- After dropping: Farmer buddy shows "Loading new skin..." message
- After loading: Skin is applied to the interface
- After loading: Farmer buddy celebrates with "Skin loaded successfully!"
- Configuration is updated with the new skin path

### 3. Testing Command-Line Arguments (Development)

**Prerequisites:**

- Application built in development mode

**Steps:**

1. Build the application:

   ```bash
   pnpm tauri build --debug
   ```

2. Run the application with a skin file path as an argument:

   ```bash
   # Windows
   ./src-tauri/target/debug/milk.exe "C:\path\to\skin.wsz"

   # macOS/Linux (if supported in future)
   ./src-tauri/target/debug/milk "/path/to/skin.wsz"
   ```

3. Check the application logs for file association events

**Expected Behavior:**

- Application launches
- Log shows: "Received file argument: [path]"
- Log shows: "Detected skin file, will load on frontend"
- Skin is loaded and applied automatically

## Error Handling

The implementation includes comprehensive error handling:

1. **Invalid Skin Files**: Falls back to default skin and shows error via farmer
2. **Missing Files**: Shows error message via farmer buddy
3. **Corrupted Files**: Falls back to default skin with error notification
4. **Permission Issues**: Displays appropriate error message

## Validation Checklist

- [ ] MSI installer registers file associations correctly
- [ ] Double-clicking .wsz file launches application and loads skin
- [ ] Double-clicking .wal file launches application and loads skin
- [ ] Right-click "Open with milk" works for both file types
- [ ] Drag-and-drop shows visual feedback
- [ ] Drag-and-drop loads skin successfully
- [ ] Farmer buddy provides appropriate feedback during loading
- [ ] Configuration is persisted after skin loading
- [ ] Error handling works for invalid files
- [ ] Application logs file association events

## Known Limitations

1. File associations are only configured for Windows (via WiX)
2. macOS and Linux file associations would require platform-specific configuration
3. Drag-and-drop requires the file path to be accessible (local files only)

## Related Files

- `src-tauri/wix/file-associations.wxs` - WiX file association configuration
- `src-tauri/tauri.conf.json` - Tauri configuration with drag-drop enabled
- `src-tauri/src/lib.rs` - Command-line argument handling
- `src/routes/+page.svelte` - Drag-and-drop implementation and event handling
- `src/lib/tauri/ipc.ts` - IPC functions for skin loading

## Requirements Validated

This implementation validates **Requirement 4.1**:

- WHEN a user selects a `.wsz` skin file THEN the Tauri Application SHALL extract the archive and parse the skin assets
