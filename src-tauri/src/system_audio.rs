use crate::error::MilkError;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

/// System audio capture state
pub struct SystemAudioCapture {
    #[cfg(target_os = "windows")]
    stream: Option<cpal::Stream>,
    is_active: Arc<Mutex<bool>>,
}

impl SystemAudioCapture {
    pub fn new() -> Self {
        Self {
            #[cfg(target_os = "windows")]
            stream: None,
            is_active: Arc::new(Mutex::new(false)),
        }
    }

    /// Start capturing system audio (loopback recording on Windows)
    pub fn start(&mut self, app_handle: AppHandle) -> std::result::Result<(), MilkError> {
        #[cfg(target_os = "windows")]
        {
            // Check if already active
            {
                let is_active = self.is_active.lock().unwrap();
                if *is_active {
                    return Ok(());
                }
            }

            // Get the default host
            let host = cpal::default_host();

            // Try to get loopback device (Windows WASAPI)
            let device = {
                // On Windows, we need to use the loopback device
                // This captures all system audio output
                host.default_output_device()
                    .ok_or_else(|| MilkError::SystemAudio("No output device found".to_string()))?
            };

            // Get the default config
            let config = device
                .default_input_config()
                .map_err(|e| MilkError::SystemAudio(format!("Failed to get default config: {}", e)))?;

            let is_active = Arc::clone(&self.is_active);

            // Build the input stream
            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => self.build_stream::<f32>(&device, &config.into(), app_handle, is_active)?,
                cpal::SampleFormat::I16 => self.build_stream::<i16>(&device, &config.into(), app_handle, is_active)?,
                cpal::SampleFormat::U16 => self.build_stream::<u16>(&device, &config.into(), app_handle, is_active)?,
                _ => {
                    return Err(MilkError::SystemAudio(
                        "Unsupported sample format".to_string(),
                    ))
                }
            };

            // Start the stream
            stream
                .play()
                .map_err(|e| MilkError::SystemAudio(format!("Failed to start stream: {}", e)))?;

            self.stream = Some(stream);
            *self.is_active.lock().unwrap() = true;

            Ok(())
        }

        #[cfg(not(target_os = "windows"))]
        {
            // System audio capture is only supported on Windows
            Err(MilkError::SystemAudio(
                "System audio capture is only supported on Windows".to_string(),
            ))
        }
    }

    /// Stop capturing system audio
    pub fn stop(&mut self) -> std::result::Result<(), MilkError> {
        #[cfg(target_os = "windows")]
        {
            if let Some(stream) = self.stream.take() {
                drop(stream);
            }
        }
        *self.is_active.lock().unwrap() = false;
        Ok(())
    }

    /// Check if capture is active
    pub fn is_active(&self) -> bool {
        *self.is_active.lock().unwrap()
    }

    /// Build an input stream for a specific sample format
    #[cfg(target_os = "windows")]
    fn build_stream<T>(
        &self,
        device: &cpal::Device,
        config: &cpal::StreamConfig,
        app_handle: AppHandle,
        is_active: Arc<Mutex<bool>>,
    ) -> std::result::Result<cpal::Stream, MilkError>
    where
        T: cpal::Sample + cpal::SizedSample,
        f32: From<T>,
    {
        let channels = config.channels as usize;
        let sample_rate = config.sample_rate.0;

        // Buffer to accumulate samples for FFT
        let buffer_size = 2048; // Match FFT size in visualizer
        let buffer = Arc::new(Mutex::new(Vec::with_capacity(buffer_size)));

        let stream = device
            .build_input_stream(
                config,
                move |data: &[T], _: &cpal::InputCallbackInfo| {
                    let active = *is_active.lock().unwrap();
                    if !active {
                        return;
                    }

                    // Convert samples to f32 and mix down to mono
                    let mut buffer = buffer.lock().unwrap();
                    
                    for chunk in data.chunks(channels) {
                        // Mix down to mono by averaging channels
                        let mono_sample: f32 = chunk
                            .iter()
                            .map(|&s| f32::from(s))
                            .sum::<f32>()
                            / channels as f32;
                        
                        buffer.push(mono_sample);

                        // When buffer is full, send to frontend
                        if buffer.len() >= buffer_size {
                            let audio_data: Vec<f32> = buffer.drain(..).collect();
                            
                            // Emit event to frontend with audio data
                            let _ = app_handle.emit("system-audio-data", SystemAudioData {
                                samples: audio_data,
                                sample_rate,
                            });
                        }
                    }
                },
                move |err| {
                    eprintln!("System audio capture error: {}", err);
                },
                None,
            )
            .map_err(|e| MilkError::SystemAudio(format!("Failed to build stream: {}", e)))?;

        Ok(stream)
    }
}

/// Audio data payload sent to frontend
#[derive(Clone, serde::Serialize)]
pub struct SystemAudioData {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

/// Tauri command to start system audio capture
#[tauri::command]
pub async fn start_system_audio_capture(
    app_handle: AppHandle,
    state: tauri::State<'_, SystemAudioCaptureState>,
) -> std::result::Result<(), String> {
    let mut capture = state.0.lock().unwrap();
    capture.start(app_handle).map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to stop system audio capture
#[tauri::command]
pub async fn stop_system_audio_capture(
    state: tauri::State<'_, SystemAudioCaptureState>,
) -> std::result::Result<(), String> {
    let mut capture = state.0.lock().unwrap();
    capture.stop().map_err(|e| e.to_string())?;
    Ok(())
}

/// Tauri command to check if system audio capture is active
#[tauri::command]
pub async fn is_system_audio_capture_active(
    state: tauri::State<'_, SystemAudioCaptureState>,
) -> std::result::Result<bool, String> {
    let capture = state.0.lock().unwrap();
    Ok(capture.is_active())
}

/// Wrapper type for Tauri state management
pub struct SystemAudioCaptureState(pub Arc<Mutex<SystemAudioCapture>>);

// Implement Send + Sync for the wrapper
unsafe impl Send for SystemAudioCaptureState {}
unsafe impl Sync for SystemAudioCaptureState {}
