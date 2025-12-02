<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  import { startSystemAudioCapture, stopSystemAudioCapture } from '$lib/tauri/ipc';

  // Props
  let {
    audioElement = $bindable(null),
    style = $bindable('bars' as 'bars' | 'waveform' | 'spectrum'),
    width = 800,
    height = 200,
    useSystemAudio = false
  }: {
    audioElement?: HTMLAudioElement | null;
    style?: 'bars' | 'waveform' | 'spectrum';
    width?: number;
    height?: number;
    useSystemAudio?: boolean;
  } = $props();

  // Canvas and Web Audio API
  let canvasElement: HTMLCanvasElement | null = null;
  let canvasContext: CanvasRenderingContext2D | null = null;
  let audioContext: AudioContext | null = null;
  let analyzerNode: AnalyserNode | null = null;
  let sourceNode: MediaElementAudioSourceNode | null = null;
  let animationFrameId: number | null = null;
  let isActive = false;
  let isWindowFocused = true;
  let isWindowVisible = true;
  let lastFrameTime = 0;
  const TARGET_FPS = 30;
  const TARGET_FPS_UNFOCUSED = 10;
  const TARGET_FPS_HIDDEN = 5;
  const FRAME_INTERVAL = 1000 / TARGET_FPS;
  const FRAME_INTERVAL_UNFOCUSED = 1000 / TARGET_FPS_UNFOCUSED;
  const FRAME_INTERVAL_HIDDEN = 1000 / TARGET_FPS_HIDDEN;
  
  // System audio capture
  let systemAudioUnlisten: (() => void) | null = null;
  let systemAudioBuffer: Float32Array | null = null;

  // FFT configuration
  const FFT_SIZE = 2048;
  const SMOOTHING = 0.8;

  // Initialize Web Audio API
  async function initializeAudioContext() {
    if (audioContext) return;

    try {
      audioContext = new AudioContext();
      analyzerNode = audioContext.createAnalyser();
      analyzerNode.fftSize = FFT_SIZE;
      analyzerNode.smoothingTimeConstant = SMOOTHING;

      if (useSystemAudio) {
        // For streaming services, use backend system audio capture
        try {
          await startSystemAudioCapture();
          
          // Listen for system audio data events from backend
          systemAudioUnlisten = await listen<{ samples: number[], sample_rate: number }>('system-audio-data', (event) => {
            // Store the audio samples for visualization
            systemAudioBuffer = new Float32Array(event.payload.samples);
          });
          
          console.log('System audio capture started successfully');
        } catch (err) {
          console.warn('System audio capture not available, using fallback visualization:', err);
          // Fall back to silent mode - visualizer will show but won't have data
        }
      } else if (audioElement) {
        // For local playback, use audio element
        sourceNode = audioContext.createMediaElementSource(audioElement);
        sourceNode.connect(analyzerNode);
        analyzerNode.connect(audioContext.destination);
      }

      isActive = true;
    } catch (error) {
      console.error('Failed to initialize Web Audio API:', error);
    }
  }

  // Start visualization
  export async function start() {
    if (!canvasElement) return;
    if (!useSystemAudio && !audioElement) return;

    // Initialize audio context if not already done
    if (!audioContext) {
      await initializeAudioContext();
    }

    // Resume audio context if suspended
    if (audioContext && audioContext.state === 'suspended') {
      await audioContext.resume();
    }

    // Start rendering loop
    if (!animationFrameId) {
      render(0);
    }
  }

  // Stop visualization
  export async function stop() {
    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }

    // Clear canvas
    if (canvasContext) {
      canvasContext.clearRect(0, 0, width, height);
    }

    // Stop system audio capture if active
    if (useSystemAudio && systemAudioUnlisten) {
      try {
        await stopSystemAudioCapture();
        systemAudioUnlisten();
        systemAudioUnlisten = null;
        systemAudioBuffer = null;
      } catch (err) {
        console.warn('Failed to stop system audio capture:', err);
      }
    }

    // Mark as inactive
    isActive = false;
  }

  // Set visualization style
  export function setStyle(newStyle: 'bars' | 'waveform' | 'spectrum') {
    style = newStyle;
  }

  // Rendering loop with adaptive throttling
  function render(timestamp: number = 0) {
    if (!canvasContext) return;

    animationFrameId = requestAnimationFrame(render);

    // Adaptive throttling based on window state
    if (lastFrameTime > 0) {
      const elapsed = timestamp - lastFrameTime;
      
      // Determine target frame interval based on window state
      let targetInterval = FRAME_INTERVAL;
      if (!isWindowVisible) {
        // Minimal rendering when hidden (5 FPS)
        targetInterval = FRAME_INTERVAL_HIDDEN;
      } else if (!isWindowFocused) {
        // Reduced rendering when not focused (10 FPS)
        targetInterval = FRAME_INTERVAL_UNFOCUSED;
      }
      
      // Skip frame if not enough time has elapsed
      if (elapsed < targetInterval) {
        return;
      }
    }

    lastFrameTime = timestamp;

    // Use system audio buffer if available, otherwise use analyzer node
    if (useSystemAudio && systemAudioBuffer) {
      // Render from system audio buffer
      if (style === 'waveform') {
        renderWaveformFromBuffer(systemAudioBuffer);
      } else {
        // For bars and spectrum, we need frequency data
        // Convert time domain to frequency domain using FFT
        const frequencyData = computeFFT(systemAudioBuffer);
        if (style === 'bars') {
          renderBars(frequencyData);
        } else if (style === 'spectrum') {
          renderSpectrum(frequencyData);
        }
      }
    } else if (analyzerNode) {
      // Get frequency or time domain data based on style
      if (style === 'waveform') {
        const bufferLength = analyzerNode.fftSize;
        const dataArray = new Uint8Array(bufferLength);
        analyzerNode.getByteTimeDomainData(dataArray);
        renderWaveform(dataArray);
      } else {
        const bufferLength = analyzerNode.frequencyBinCount;
        const dataArray = new Uint8Array(bufferLength);
        analyzerNode.getByteFrequencyData(dataArray);
        
        if (style === 'bars') {
          renderBars(dataArray);
        } else if (style === 'spectrum') {
          renderSpectrum(dataArray);
        }
      }
    }
  }

  // Simple FFT approximation for system audio buffer
  function computeFFT(buffer: Float32Array): Uint8Array {
    // For simplicity, we'll compute energy in frequency bands
    // This is a simplified approach - a real FFT would be more accurate
    const frequencyBins = 128;
    const result = new Uint8Array(frequencyBins);
    const samplesPerBin = Math.floor(buffer.length / frequencyBins);
    
    for (let i = 0; i < frequencyBins; i++) {
      let sum = 0;
      const start = i * samplesPerBin;
      const end = Math.min(start + samplesPerBin, buffer.length);
      
      for (let j = start; j < end; j++) {
        sum += Math.abs(buffer[j]);
      }
      
      const average = sum / samplesPerBin;
      result[i] = Math.min(255, Math.floor(average * 255 * 2)); // Scale up for visibility
    }
    
    return result;
  }

  // Render waveform from Float32Array buffer
  function renderWaveformFromBuffer(buffer: Float32Array) {
    if (!canvasContext) return;

    canvasContext.clearRect(0, 0, width, height);
    canvasContext.lineWidth = 2;
    canvasContext.strokeStyle = '#00aaff';
    canvasContext.beginPath();

    const sliceWidth = width / buffer.length;
    let x = 0;

    for (let i = 0; i < buffer.length; i++) {
      const v = (buffer[i] + 1) / 2; // Normalize from [-1, 1] to [0, 1]
      const y = v * height;

      if (i === 0) {
        canvasContext.moveTo(x, y);
      } else {
        canvasContext.lineTo(x, y);
      }

      x += sliceWidth;
    }

    canvasContext.lineTo(width, height / 2);
    canvasContext.stroke();
  }

  // Render bars visualization
  function renderBars(dataArray: Uint8Array) {
    if (!canvasContext) return;

    canvasContext.clearRect(0, 0, width, height);

    const barCount = 64;
    const barWidth = width / barCount;
    const step = Math.floor(dataArray.length / barCount);

    for (let i = 0; i < barCount; i++) {
      const value = dataArray[i * step];
      const barHeight = (value / 255) * height;
      const x = i * barWidth;
      const y = height - barHeight;

      // Gradient color based on height
      const hue = (value / 255) * 120; // 0 (red) to 120 (green)
      canvasContext.fillStyle = `hsl(${hue}, 100%, 50%)`;
      canvasContext.fillRect(x, y, barWidth - 2, barHeight);
    }
  }

  // Render waveform visualization
  function renderWaveform(dataArray: Uint8Array) {
    if (!canvasContext) return;

    canvasContext.clearRect(0, 0, width, height);
    canvasContext.lineWidth = 2;
    canvasContext.strokeStyle = '#00aaff';
    canvasContext.beginPath();

    const sliceWidth = width / dataArray.length;
    let x = 0;

    for (let i = 0; i < dataArray.length; i++) {
      const v = dataArray[i] / 128.0;
      const y = (v * height) / 2;

      if (i === 0) {
        canvasContext.moveTo(x, y);
      } else {
        canvasContext.lineTo(x, y);
      }

      x += sliceWidth;
    }

    canvasContext.lineTo(width, height / 2);
    canvasContext.stroke();
  }

  // Render spectrum visualization
  function renderSpectrum(dataArray: Uint8Array) {
    if (!canvasContext) return;

    canvasContext.clearRect(0, 0, width, height);

    const barCount = 128;
    const barWidth = width / barCount;
    const step = Math.floor(dataArray.length / barCount);

    for (let i = 0; i < barCount; i++) {
      const value = dataArray[i * step];
      const barHeight = (value / 255) * height;
      const x = i * barWidth;
      const y = height - barHeight;

      // Color gradient from blue to red
      const hue = 240 - (value / 255) * 240;
      canvasContext.fillStyle = `hsl(${hue}, 100%, 50%)`;
      canvasContext.fillRect(x, y, barWidth - 1, barHeight);
    }
  }

  // Watch for audio element changes
  $effect(() => {
    if (audioElement && !audioContext) {
      // Audio element is available, but context not initialized yet
      // Will be initialized when start() is called
    }
  });

  // Watch for style changes
  $effect(() => {
    // Style changed, no need to restart - will use new style on next frame
    style;
  });

  onMount(() => {
    if (canvasElement) {
      canvasContext = canvasElement.getContext('2d');
    }

    // Listen for window focus/blur events to optimize rendering
    const handleFocus = () => {
      isWindowFocused = true;
      console.log('[Visualizer] Window focused - increasing frame rate to 30 FPS');
    };
    const handleBlur = () => {
      isWindowFocused = false;
      console.log('[Visualizer] Window blurred - reducing frame rate to 10 FPS');
    };

    // Listen for visibility changes to further optimize
    const handleVisibilityChange = () => {
      isWindowVisible = !document.hidden;
      if (document.hidden) {
        console.log('[Visualizer] Window hidden - reducing frame rate to 5 FPS');
      } else {
        console.log('[Visualizer] Window visible - restoring frame rate');
      }
    };

    window.addEventListener('focus', handleFocus);
    window.addEventListener('blur', handleBlur);
    document.addEventListener('visibilitychange', handleVisibilityChange);

    return () => {
      window.removeEventListener('focus', handleFocus);
      window.removeEventListener('blur', handleBlur);
      document.removeEventListener('visibilitychange', handleVisibilityChange);
    };
  });

  onDestroy(async () => {
    await stop();
    
    // Clean up Web Audio API resources
    if (sourceNode) {
      sourceNode.disconnect();
    }
    if (analyzerNode) {
      analyzerNode.disconnect();
    }
    if (audioContext) {
      audioContext.close();
    }
    
    // Clean up system audio listener
    if (systemAudioUnlisten) {
      systemAudioUnlisten();
    }
  });

  // Export active state for testing
  export function isVisualizerActive(): boolean {
    return isActive;
  }

  // Export analyzer node for testing
  export function getAnalyzerNode(): AnalyserNode | null {
    return analyzerNode;
  }
</script>

<div class="visualizer">
  <canvas
    bind:this={canvasElement}
    {width}
    {height}
    class="visualizer-canvas"
  ></canvas>
</div>

<style>
  .visualizer {
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: var(--color-visualizer-bg, #000000);
    padding: 8px;
    border-radius: 4px;
  }

  .visualizer-canvas {
    display: block;
    background-color: #000000;
  }
</style>
