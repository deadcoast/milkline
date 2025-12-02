import { describe, it, expect, vi, beforeEach } from 'vitest';
import {
    handleError,
    handleErrorWithRecovery,
    extractErrorMessage,
    getRecoveryStrategy,
    withErrorHandling,
    withRetry,
    isCriticalError,
    isRecoverableError,
    showSuccess
} from './errorHandler';
import { farmerStore } from '../stores/farmerStore';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/core', () => ({
    invoke: vi.fn()
}));

describe('errorHandler', () => {
    beforeEach(() => {
        vi.clearAllMocks();
    });

    describe('extractErrorMessage', () => {
        it('should extract message from string error', () => {
            const message = extractErrorMessage('Test error');
            expect(message).toBe('Test error');
        });

        it('should extract message from Error object', () => {
            const error = new Error('Test error');
            const message = extractErrorMessage(error);
            expect(message).toBe('Test error');
        });

        it('should extract message from object with message property', () => {
            const error = { message: 'Test error' };
            const message = extractErrorMessage(error);
            expect(message).toBe('Test error');
        });

        it('should return default message for unknown error type', () => {
            const message = extractErrorMessage(null);
            expect(message).toBe('An unexpected error occurred');
        });
    });

    describe('isCriticalError', () => {
        it('should identify disk full as critical', () => {
            expect(isCriticalError('Disk full error')).toBe(true);
        });

        it('should identify permission denied as critical', () => {
            expect(isCriticalError('Permission denied')).toBe(true);
        });

        it('should identify audio device unavailable as critical', () => {
            expect(isCriticalError('Audio device unavailable')).toBe(true);
        });

        it('should identify authentication failed as critical', () => {
            expect(isCriticalError('Authentication failed')).toBe(true);
        });

        it('should not identify network timeout as critical', () => {
            expect(isCriticalError('Network timeout')).toBe(false);
        });
    });

    describe('isRecoverableError', () => {
        it('should identify timeout as recoverable', () => {
            expect(isRecoverableError('Connection timeout')).toBe(true);
        });

        it('should identify rate limit as recoverable', () => {
            expect(isRecoverableError('Rate limit exceeded')).toBe(true);
        });

        it('should identify corrupted file as recoverable', () => {
            expect(isRecoverableError('File corrupted')).toBe(true);
        });

        it('should identify skin error as recoverable', () => {
            expect(isRecoverableError('Skin parse error')).toBe(true);
        });

        it('should identify metadata error as recoverable', () => {
            expect(isRecoverableError('Metadata extraction failed')).toBe(true);
        });

        it('should not identify disk full as recoverable', () => {
            expect(isRecoverableError('Disk full')).toBe(false);
        });
    });

    describe('getRecoveryStrategy', () => {
        it('should provide recovery strategy for network errors', async () => {
            const strategy = await getRecoveryStrategy('Network timeout');
            expect(strategy.canRecover).toBe(true);
            expect(strategy.suggestion).toContain('internet');
        });

        it('should provide recovery strategy for authentication errors', async () => {
            const strategy = await getRecoveryStrategy('Authentication failed');
            expect(strategy.canRecover).toBe(true);
            expect(strategy.suggestion).toContain('log in');
        });

        it('should provide recovery strategy for config errors', async () => {
            const strategy = await getRecoveryStrategy('Config corrupted');
            expect(strategy.canRecover).toBe(true);
            expect(strategy.suggestion).toContain('configuration');
        });

        it('should provide recovery strategy for rate limit', async () => {
            const strategy = await getRecoveryStrategy('Rate limit exceeded');
            expect(strategy.canRecover).toBe(true);
            expect(strategy.suggestion).toContain('wait');
        });

        it('should indicate disk full is not recoverable', async () => {
            const strategy = await getRecoveryStrategy('Disk full');
            expect(strategy.canRecover).toBe(false);
            expect(strategy.suggestion).toContain('disk space');
        });

        it('should indicate permission denied is not recoverable', async () => {
            const strategy = await getRecoveryStrategy('Permission denied');
            expect(strategy.canRecover).toBe(false);
            expect(strategy.suggestion).toContain('permission');
        });
    });

    describe('withErrorHandling', () => {
        it('should return result on success', async () => {
            const operation = vi.fn().mockResolvedValue('success');
            const result = await withErrorHandling(operation, 'Test');
            expect(result).toBe('success');
        });

        it('should call onSuccess callback', async () => {
            const operation = vi.fn().mockResolvedValue('success');
            const onSuccess = vi.fn();
            await withErrorHandling(operation, 'Test', onSuccess);
            expect(onSuccess).toHaveBeenCalledWith('success');
        });

        it('should return null on error', async () => {
            const operation = vi.fn().mockRejectedValue(new Error('Test error'));
            const result = await withErrorHandling(operation, 'Test');
            expect(result).toBeNull();
        });
    });

    describe('withRetry', () => {
        it('should return result on first success', async () => {
            const operation = vi.fn().mockResolvedValue('success');
            const result = await withRetry(operation, 'Test', 3, 10);
            expect(result).toBe('success');
            expect(operation).toHaveBeenCalledTimes(1);
        });

        it('should retry on recoverable error', async () => {
            const operation = vi.fn()
                .mockRejectedValueOnce(new Error('Network timeout'))
                .mockResolvedValueOnce('success');
            
            const result = await withRetry(operation, 'Test', 3, 10);
            expect(result).toBe('success');
            expect(operation).toHaveBeenCalledTimes(2);
        });

        it('should not retry on non-recoverable error', async () => {
            const operation = vi.fn().mockRejectedValue(new Error('Disk full'));
            const result = await withRetry(operation, 'Test', 3, 10);
            expect(result).toBeNull();
            expect(operation).toHaveBeenCalledTimes(1);
        });

        it('should return null after max retries', async () => {
            const operation = vi.fn().mockRejectedValue(new Error('Network timeout'));
            const result = await withRetry(operation, 'Test', 3, 10);
            expect(result).toBeNull();
            expect(operation).toHaveBeenCalledTimes(3);
        });
    });

    describe('handleError', () => {
        it('should show error via farmer', () => {
            const showErrorSpy = vi.spyOn(farmerStore, 'showError');
            handleError('Test error', 'Test context');
            expect(showErrorSpy).toHaveBeenCalledWith('Test error');
        });

        it('should mark critical errors with warning icon', () => {
            const showErrorSpy = vi.spyOn(farmerStore, 'showError');
            handleError('Disk full', 'Test context');
            expect(showErrorSpy).toHaveBeenCalledWith('⚠️ Disk full');
        });
    });

    describe('showSuccess', () => {
        it('should show success message via farmer', () => {
            const celebrateSpy = vi.spyOn(farmerStore, 'celebrate');
            showSuccess('Success!', 2000);
            expect(celebrateSpy).toHaveBeenCalledWith('Success!', 2000);
        });
    });
});
