<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { farmerStore, playerStore } from '../stores';
    import { initializeFarmerPlayerSync } from '../stores/farmerPlayerSync';
    import type { FarmerStateData, PlayerState } from '../types';

    // Props
    let {
        analyzerNode = $bindable(null)
    }: {
        analyzerNode?: AnalyserNode | null;
    } = $props();

    let farmerState: FarmerStateData;
    let playerState: PlayerState;
    let blinkInterval: number | undefined;
    let lookInterval: number | undefined;
    let listeningAnimationFrame: number | undefined;
    let audioReactiveOffset = 0;
    let bobPhase = 0;

    // Subscribe to stores
    const unsubscribeFarmer = farmerStore.subscribe(state => {
        farmerState = state;
    });

    const unsubscribePlayer = playerStore.subscribe(state => {
        const wasPlaying = playerState?.isPlaying;
        playerState = state;

        // Start/stop listening animations based on playback state
        if (state.isPlaying && !wasPlaying) {
            startListeningAnimations();
        } else if (!state.isPlaying && wasPlaying) {
            stopListeningAnimations();
        }
    });

    // Start listening state animations synchronized with audio
    function startListeningAnimations() {
        stopListeningAnimations(); // Clear any existing animation

        function animate() {
            if (farmerState.currentState !== 'listening') {
                stopListeningAnimations();
                return;
            }

            // Get audio data if analyzer is available
            let audioEnergy = 0;
            if (analyzerNode) {
                const dataArray = new Uint8Array(analyzerNode.frequencyBinCount);
                analyzerNode.getByteFrequencyData(dataArray);
                
                // Calculate average energy in lower frequencies (bass/rhythm)
                const bassRange = Math.floor(dataArray.length * 0.1);
                let sum = 0;
                for (let i = 0; i < bassRange; i++) {
                    sum += dataArray[i];
                }
                audioEnergy = sum / bassRange / 255; // Normalize to 0-1
            }

            // Update bob phase for continuous movement
            bobPhase += 0.05 + audioEnergy * 0.1;
            
            // Calculate audio-reactive offset
            audioReactiveOffset = Math.sin(bobPhase) * (2 + audioEnergy * 3);

            // Randomly change mouth expression to simulate "singing along"
            if (Math.random() < 0.05) {
                const mouthExpressions: Array<'smile' | 'talk-1' | 'talk-2'> = ['smile', 'talk-1', 'talk-2'];
                const randomMouth = mouthExpressions[Math.floor(Math.random() * mouthExpressions.length)];
                farmerStore.setExpression({ mouth: randomMouth });
            }

            // Occasionally look around while listening
            if (Math.random() < 0.01) {
                const directions: Array<'look-left' | 'look-right' | 'neutral'> = ['look-left', 'look-right', 'neutral'];
                const direction = directions[Math.floor(Math.random() * directions.length)];
                farmerStore.setExpression({ eyes: direction });
                setTimeout(() => {
                    farmerStore.setExpression({ eyes: 'neutral' });
                }, 500);
            }

            listeningAnimationFrame = requestAnimationFrame(animate);
        }

        listeningAnimationFrame = requestAnimationFrame(animate);
    }

    // Stop listening animations
    function stopListeningAnimations() {
        if (listeningAnimationFrame !== undefined) {
            cancelAnimationFrame(listeningAnimationFrame);
            listeningAnimationFrame = undefined;
        }
        audioReactiveOffset = 0;
        bobPhase = 0;
    }

    // Idle animations
    function startIdleAnimations() {
        // Blink animation - random intervals
        blinkInterval = window.setInterval(() => {
            if (farmerState.currentState === 'idle') {
                farmerStore.setExpression({ eyes: 'blink' });
                setTimeout(() => {
                    farmerStore.setExpression({ eyes: 'neutral' });
                }, 150);
            }
        }, 3000 + Math.random() * 2000);

        // Look around animation - random intervals
        lookInterval = window.setInterval(() => {
            if (farmerState.currentState === 'idle') {
                const directions: Array<'look-left' | 'look-right' | 'neutral'> = ['look-left', 'look-right', 'neutral'];
                const direction = directions[Math.floor(Math.random() * directions.length)];
                farmerStore.setExpression({ eyes: direction });
                setTimeout(() => {
                    farmerStore.setExpression({ eyes: 'neutral' });
                }, 1000);
            }
        }, 5000 + Math.random() * 3000);
    }

    function stopIdleAnimations() {
        if (blinkInterval) {
            clearInterval(blinkInterval);
            blinkInterval = undefined;
        }
        if (lookInterval) {
            clearInterval(lookInterval);
            lookInterval = undefined;
        }
    }

    // Initialize farmer-player synchronization
    const unsubscribeSync = initializeFarmerPlayerSync();

    onMount(() => {
        startIdleAnimations();
        
        // If already playing when mounted, start listening animations
        if (playerState?.isPlaying) {
            startListeningAnimations();
        }
    });

    onDestroy(() => {
        stopIdleAnimations();
        stopListeningAnimations();
        unsubscribeFarmer();
        unsubscribePlayer();
        unsubscribeSync();
    });

    // Get eye position based on expression
    function getEyeTransform(eye: 'left' | 'right'): string {
        if (farmerState.expression.eyes === 'blink') {
            return 'scaleY(0.1)';
        }
        if (farmerState.expression.eyes === 'look-left') {
            return 'translateX(-2)';
        }
        if (farmerState.expression.eyes === 'look-right') {
            return 'translateX(2)';
        }
        return '';
    }

    // Get mouth path based on expression
    function getMouthPath(): string {
        switch (farmerState.expression.mouth) {
            case 'smile':
                return 'M 30 45 Q 40 50 50 45';
            case 'talk-1':
                return 'M 35 45 Q 40 48 45 45';
            case 'talk-2':
                return 'M 35 45 Q 40 50 45 45';
            case 'neutral':
            default:
                return 'M 35 45 L 45 45';
        }
    }

    // Get state color
    function getStateColor(): string {
        switch (farmerState.currentState) {
            case 'listening':
                return '#4CAF50';
            case 'prompting':
                return '#2196F3';
            case 'celebrating':
                return '#FFC107';
            case 'error':
                return '#F44336';
            case 'idle':
            default:
                return '#9E9E9E';
        }
    }
</script>

<div class="farmer-container" style="transform: translateY({audioReactiveOffset}px);">
    <svg class="farmer-svg" viewBox="0 0 80 80" xmlns="http://www.w3.org/2000/svg">
        <!-- Face -->
        <circle cx="40" cy="40" r="30" fill="#FFE0BD" stroke={getStateColor()} stroke-width="2" />
        
        <!-- Left Eye -->
        <ellipse 
            cx="30" 
            cy="35" 
            rx="4" 
            ry="5" 
            fill="#333"
            transform={getEyeTransform('left')}
            style="transition: transform 0.15s ease;"
        />
        
        <!-- Right Eye -->
        <ellipse 
            cx="50" 
            cy="35" 
            rx="4" 
            ry="5" 
            fill="#333"
            transform={getEyeTransform('right')}
            style="transition: transform 0.15s ease;"
        />
        
        <!-- Mouth -->
        <path 
            d={getMouthPath()} 
            stroke="#333" 
            stroke-width="2" 
            fill="none"
            stroke-linecap="round"
            style="transition: d 0.2s ease;"
        />
    </svg>

    {#if farmerState.message}
        <div class="speech-bubble" class:error={farmerState.currentState === 'error'}>
            <div class="bubble-content">
                {farmerState.message}
            </div>
            <div class="bubble-arrow"></div>
        </div>
    {/if}
</div>

<style>
    .farmer-container {
        position: relative;
        width: 80px;
        height: 80px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .farmer-svg {
        width: 100%;
        height: 100%;
    }

    .speech-bubble {
        position: absolute;
        top: -60px;
        left: 50%;
        transform: translateX(-50%);
        background: white;
        border: 2px solid #333;
        border-radius: 8px;
        padding: 8px 12px;
        min-width: 120px;
        max-width: 200px;
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        z-index: 10;
        animation: fadeIn 0.3s ease;
    }

    .speech-bubble.error {
        background: #FFEBEE;
        border-color: #F44336;
    }

    .bubble-content {
        font-size: 12px;
        line-height: 1.4;
        color: #333;
        text-align: center;
    }

    .bubble-arrow {
        position: absolute;
        bottom: -8px;
        left: 50%;
        transform: translateX(-50%);
        width: 0;
        height: 0;
        border-left: 8px solid transparent;
        border-right: 8px solid transparent;
        border-top: 8px solid #333;
    }

    .bubble-arrow::before {
        content: '';
        position: absolute;
        bottom: 2px;
        left: -6px;
        width: 0;
        height: 0;
        border-left: 6px solid transparent;
        border-right: 6px solid transparent;
        border-top: 6px solid white;
    }

    .speech-bubble.error .bubble-arrow::before {
        border-top-color: #FFEBEE;
    }

    @keyframes fadeIn {
        from {
            opacity: 0;
            transform: translateX(-50%) translateY(-5px);
        }
        to {
            opacity: 1;
            transform: translateX(-50%) translateY(0);
        }
    }
</style>
