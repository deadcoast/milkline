<script lang="ts">
  import { onMount, onDestroy } from 'svelte';

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
  let lastFrameTime = 0;
  const TARGET_FPS = 30;
  const FRAME_INTERVAL = 1000 / TARGET_FPS;

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
        // For streaming services, try to capture system audio
        // Note: This requires user permission and may not work in all browsers
        try {
          const stream = await navigator.mediaDevices.getUserMedia({ 
            audio: {
              echoCancellation: false,
              noiseSuppression: false,
              autoGainControl: false
            } 
          });
          const micSource = audioContext.createMediaStreamSource(stream);
          micSource.connect(analyzerNode);
          // Don't connect to destination to avoid feedback
        } catch (err) {
          console.warn('System audio capture not available, using silent visualization:', err);
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
  export function stop() {
    if (animationFrameId !== null) {
      cancelAnimationFrame(animationFrameId);
      animationFrameId = null;
    }

    // Clear canvas
    if (canvasContext) {
      canvasContext.clearRect(0, 0, width, height);
    }

    // Mark as inactive
    isActive = false;
  }

  // Set visualization style
  export function setStyle(newStyle: 'bars' | 'waveform' | 'spectrum') {
    style = newStyle;
  }

  // Rendering loop with throttling
  function render(timestamp: number = 0) {
    if (!analyzerNode || !canvasContext) return;

    animationFrameId = requestAnimationFrame(render);

    // Throttle rendering based on window focus
    if (lastFrameTime > 0) {
      const elapsed = timestamp - lastFrameTime;
      
      if (!isWindowFocused) {
        // Reduce frame rate to 10 FPS when not focused
        if (elapsed < 100) { // 100ms = 10 FPS
          return;
        }
      } else {
        // Throttle to target FPS when focused
        if (elapsed < FRAME_INTERVAL) {
          return;
        }
      }
    }

    lastFrameTime = timestamp;

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
    };
    const handleBlur = () => {
      isWindowFocused = false;
    };

    window.addEventListener('focus', handleFocus);
    window.addEventListener('blur', handleBlur);

    return () => {
      window.removeEventListener('focus', handleFocus);
      window.removeEventListener('blur', handleBlur);
    };
  });

  onDestroy(() => {
    stop();
    
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
