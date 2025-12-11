# System Audio Capture Implementation

## Overview

This document describes the system audio capture implementation for the milk player visualizer, which enables visualization of audio from streaming services (Spotify, YouTube) when they are playing outside the application.

## Architecture

### Backend (Rust)

**Module**: `src-tauri/src/system_audio.rs`

The backend uses the `cpal` crate to capture system audio:

- **Windows**: Uses WASAPI loopback recording to capture all system audio output
- **Other platforms**: Falls back to default input device (microphone)

**Key Components**:

1. **SystemAudioCapture**: Manages the audio capture stream
   - `start()`: Initializes and starts audio capture
   - `stop()`: Stops audio capture and cleans up resources
   - `is_active()`: Checks if capture is currently active

2. **Audio Processing**:
   - Captures audio in real-time using cpal streams
   - Converts samples to f32 format
   - Mixes multi-channel audio down to mono
   - Buffers samples (2048 samples to match FFT size)
   - Emits audio data to frontend via Tauri events

3. **Tauri Commands**:
   - `start_system_audio_capture`: Starts capturing system audio
   - `stop_system_audio_capture`: Stops capturing system audio
   - `is_system_audio_capture_active`: Checks capture status

### Frontend (TypeScript/Svelte)

**Component**: `src/lib/components/Visualizer.svelte`

The visualizer component has been enhanced to support system audio:

1. **System Audio Mode** (`useSystemAudio` prop):
   - When `true`, uses backend system audio capture instead of Web Audio API
   - Listens for `system-audio-data` events from backend
   - Stores audio samples in a Float32Array buffer

2. **Visualization**:
   - **Waveform**: Renders directly from time-domain samples
   - **Bars/Spectrum**: Computes frequency data using simplified FFT

3. **Lifecycle**:
   - Starts system audio capture when visualizer starts (if `useSystemAudio` is true)
   - Stops system audio capture when visualizer stops
   - Cleans up event listeners on component destroy

**IPC Functions**: `src/lib/tauri/ipc.ts`

- `startSystemAudioCapture()`: Invokes backend command to start capture
- `stopSystemAudioCapture()`: Invokes backend command to stop capture
- `isSystemAudioCaptureActive()`: Checks if capture is active

## Usage

### In the Main Application

The visualizer automatically switches to system audio mode when streaming services are active:

```svelte
<Visualizer
  bind:this={visualizerComponent}
  {audioElement}
  style={$configStore.visualizerStyle || 'bars'}
  width={800}
  height={400}
  useSystemAudio={currentTrack?.source === 'spotify' || currentTrack?.source === 'youtube'}
/>
```

### Behavior

1. **Local Playback**: Uses Web Audio API with the HTML5 audio element
2. **Streaming Services**: Uses system audio capture to visualize external audio

### Fallback

If system audio capture fails (e.g., no audio device, permissions denied):

- The visualizer continues to render but may show minimal activity
- Error is logged to console but doesn't break the application
- User experience degrades gracefully

## Platform Support

### Windows

- ✅ Full support via WASAPI loopback
- Captures all system audio output
- No additional permissions required

### macOS

- ⚠️ Limited support (falls back to microphone input)
- Requires user permission for microphone access
- Does not capture system audio directly

### Linux

- ⚠️ Limited support (falls back to microphone input)
- Requires user permission for microphone access
- Does not capture system audio directly

## Performance Considerations

1. **Buffer Size**: 2048 samples matches the FFT size in the visualizer
2. **Event Frequency**: Audio data is sent to frontend when buffer is full
3. **Memory**: Minimal overhead - only one buffer in flight at a time
4. **CPU**: Audio processing happens in separate thread (cpal stream callback)

## Testing

To test system audio capture:

1. Start the application
2. Play audio from Spotify or YouTube (external to the app)
3. The visualizer should show activity synchronized with the audio
4. Stop playback - visualizer should return to idle state

## Limitations

1. **Platform-specific**: Full functionality only on Windows
2. **Latency**: Small delay (~50-100ms) between audio and visualization
3. **Simplified FFT**: Uses energy-based frequency approximation, not true FFT
4. **Mono only**: Multi-channel audio is mixed down to mono

## Future Improvements

1. Implement true FFT for more accurate frequency visualization
2. Add support for macOS system audio capture (requires additional libraries)
3. Add support for Linux system audio capture (PulseAudio/PipeWire)
4. Add user controls for audio capture device selection
5. Add latency compensation
6. Support stereo visualization
