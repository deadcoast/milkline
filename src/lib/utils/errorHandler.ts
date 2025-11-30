// Error handling utilities for connecting backend errors to farmer state
import { farmerStore } from '../stores/farmerStore';

/**
 * Handle an error by showing it via farmer and logging to console
 */
export function handleError(error: unknown, context: string = 'Operation'): void {
    const errorMessage = extractErrorMessage(error);
    const fullMessage = `${context}: ${errorMessage}`;
    
    console.error(fullMessage, error);
    farmerStore.showError(errorMessage);
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
        handleError(error, context);
        return null;
    }
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
        message.includes('metadata')
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
