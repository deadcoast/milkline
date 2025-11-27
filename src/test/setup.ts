// Test setup file
import { beforeAll } from 'vitest';

beforeAll(() => {
    // Mock Tauri API for tests
    (global as any).__TAURI_INTERNALS__ = {
        invoke: async () => ({}),
    };
});
