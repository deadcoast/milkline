// Error handling utilities for connecting backend errors to farmer state
import { farmerStore } from '../stores/farmerStore';
import { invoke } from '@tauri-apps/api/core';

/**
 * Error categories matching backend MilkError types
 */
export enum ErrorCategory {
    FileSystem = 'FileSystem',
    Network = 'Network',
    Playback = 'Playback',
    Configuration = 'Configuration',
    Skin = 'Skin',
    Metadata = 'Metadata',
    Playlist = 'Playlist',
    Storage = 'Storage',
    SystemAudio = 'SystemAudio',
    General = 'General'
}

/**
 * Error recovery strategies
 */
export interface RecoveryStrategy {
    canRecover: boolean;
    suggestion: string;
    action?: () => Promise<void>;
}

/**
 * Handle an error by showing it via farmer and logging to console
 */
export function handleError(error: unknown, context: string = 'Operation'): void {
    const errorMessage = extractErrorMessage(error);
    const fullMessage = `${context}: ${errorMessage}`;
    
    console.error(fullMessage, error);
    
    // Check if error is critical
    if (isCriticalError(error)) {
        farmerStore.showError(`⚠️ ${errorMessage}`);
    } else {
        farmerStore.showError(errorMessage);
    }
}

/**
 * Handle an error with recovery attempt
 */
export async function handleErrorWithRecovery(
    error: unknown,
    context: string,
    onRecovery?: () => Promise<void>
): Promise<boolean> {
    const errorMessage = extractErrorMessage(error);
    console.error(`${context}: ${errorMessage}`, error);
    
    const recovery = await getRecoveryStrategy(error);
    
    if (recovery.canRecover) {
        farmerStore.showError(`${errorMessage}\n\n${recovery.suggestion}`);
        
        if (recovery.action) {
            try {
                await recovery.action();
                if (onRecovery) {
                    await onRecovery();
                }
                farmerStore.celebrate('Recovered successfully!');
                return true;
            } catch (recoveryError) {
                console.error('Recovery failed:', recoveryError);
                farmerStore.showError('Recovery failed. Please try again manually.');
                return false;
            }
        }
    } else {
        farmerStore.showError(`${errorMessage}\n\n${recovery.suggestion}`);
    }
    
    return false;
}

/**
 * Extract a user-friendly error message from various error types
 */
export function extractErrorMessage(error: unknown): string {
    if (typeof error === 'string') {
        return error;
    }
    
    if (error instanceof Error) {
        return error.message;
    }
    
    if (error && typeof error === 'object' && 'message' in error) {
        return String(error.message);
    }
    
    return 'An unexpected error occurred';
}

/**
 * Get recovery strategy for an error
 */
export async function getRecoveryStrategy(error: unknown): Promise<RecoveryStrategy> {
    const message = extractErrorMessage(error).toLowerCase();
    
    // Network errors - suggest retry
    if (message.includes('timeout') || message.includes('network')) {
        return {
            canRecover: true,
            suggestion: 'Check your internet connection and try again.',
            action: async () => {
                // Wait a moment before retry
                await new Promise(resolve => setTimeout(resolve, 2000));
            }
        };
    }
    
    // Authentication errors - suggest re-login
    if (message.includes('authentication') || message.includes('token expired')) {
        return {
            canRecover: true,
            suggestion: 'Please log in again to refresh your credentials.',
            action: async () => {
                // Trigger re-authentication flow
                // This would be handled by the calling code
            }
        };
    }
    
    // Config errors - auto-recover
    if (message.includes('config') && (message.includes('corrupted') || message.includes('parse'))) {
        return {
            canRecover: true,
            suggestion: "I'll create a fresh configuration for you.",
            action: async () => {
                // Backend will handle config recovery
                await invoke('load_config');
            }
        };
    }
    
    // Rate limit - suggest waiting
    if (message.includes('rate limit')) {
        return {
            canRecover: true,
            suggestion: "Too many requests. Let's wait a moment.",
            action: async () => {
                await new Promise(resolve => setTimeout(resolve, 60000));
            }
        };
    }
    
    // Skin errors - use default
    if (message.includes('skin')) {
        return {
            canRecover: true,
            suggestion: "I'll use the default look instead.",
            action: async () => {
                // Default skin will be applied automatically
            }
        };
    }
    
    // Non-recoverable errors
    if (message.includes('disk full')) {
        return {
            canRecover: false,
            suggestion: 'Free up some disk space and try again.'
        };
    }
    
    if (message.includes('permission denied')) {
        return {
            canRecover: false,
            suggestion: 'Please check file permissions or run as administrator.'
        };
    }
    
    if (message.includes('audio device')) {
        return {
            canRecover: false,
            suggestion: 'Please check your speakers or headphones.'
        };
    }
    
    // Default recovery strategy
    return {
        canRecover: isRecoverableError(error),
        suggestion: "Let's try that again."
    };
}

/**
 * Wrap an async operation with error handling
 */
export async function withErrorHandling<T>(
    operation: () => Promise<T>,
    context: string,
    onSuccess?: (result: T) => void
): Promise<T | null> {
    try {
        const result = await operation();
        if (onSuccess) {
            onSuccess(result);
        }
        return result;
    } catch (error) {
        await handleErrorWithRecovery(error, context);
        return null;
    }
}

/**
 * Wrap an async operation with retry logic
 */
export async function withRetry<T>(
    operation: () => Promise<T>,
    context: string,
    maxRetries: number = 3,
    delayMs: number = 1000
): Promise<T | null> {
    let lastError: unknown = null;
    
    for (let attempt = 0; attempt < maxRetries; attempt++) {
        try {
            return await operation();
        } catch (error) {
            lastError = error;
            
            // Check if error is recoverable
            if (!isRecoverableError(error)) {
                handleError(error, context);
                return null;
            }
            
            if (attempt < maxRetries - 1) {
                console.warn(`${context} failed (attempt ${attempt + 1}/${maxRetries}), retrying...`);
                await new Promise(resolve => setTimeout(resolve, delayMs * Math.pow(2, attempt)));
            }
        }
    }
    
    // All retries exhausted
    handleError(lastError, `${context} (after ${maxRetries} attempts)`);
    return null;
}

/**
 * Check if an error is critical (requires immediate user attention)
 */
export function isCriticalError(error: unknown): boolean {
    const message = extractErrorMessage(error).toLowerCase();
    
    return (
        message.includes('disk full') ||
        message.includes('permission denied') ||
        message.includes('audio device unavailable') ||
        message.includes('authentication failed')
    );
}

/**
 * Check if an error is recoverable (can be handled gracefully)
 */
export function isRecoverableError(error: unknown): boolean {
    const message = extractErrorMessage(error).toLowerCase();
    
    return (
        message.includes('timeout') ||
        message.includes('rate limit') ||
        message.includes('corrupted') ||
        message.includes('skin') ||
        message.includes('metadata') ||
        message.includes('network')
    );
}

/**
 * Log an error without showing it to the user (for non-critical errors)
 */
export function logError(error: unknown, context: string = 'Operation'): void {
    const errorMessage = extractErrorMessage(error);
    console.error(`${context}: ${errorMessage}`, error);
}

/**
 * Show a success message via farmer
 */
export function showSuccess(message: string, duration: number = 2000): void {
    farmerStore.celebrate(message, duration);
}

/**
 * Get error category from backend
 */
export async function getErrorCategory(errorMessage: string): Promise<ErrorCategory> {
    try {
        const category = await invoke<string>('get_error_category', { errorMsg: errorMessage });
        return category as ErrorCategory;
    } catch {
        return ErrorCategory.General;
    }
}

/**
 * Check if error type is critical via backend
 */
export async function checkIfCritical(errorType: string): Promise<boolean> {
    try {
        return await invoke<boolean>('is_error_critical', { errorType });
    } catch {
        return false;
    }
}

/**
 * Check if error type is recoverable via backend
 */
export async function checkIfRecoverable(errorType: string): Promise<boolean> {
    try {
        return await invoke<boolean>('is_error_recoverable', { errorType });
    } catch {
        return false;
    }
}
