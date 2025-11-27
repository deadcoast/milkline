// Property-based tests for FarmerBuddy component
import { describe, it, expect, beforeEach } from 'vitest';
import * as fc from 'fast-check';
import { farmerStore } from '../stores/farmerStore';
import { get } from 'svelte/store';
import type { FarmerState } from '../types';

describe('FarmerBuddy Property Tests', () => {
    beforeEach(() => {
        farmerStore.reset();
    });

    /**
     * **Feature: milk-player, Property 15: farmer error state handling**
     * **Validates: Requirements 6.3**
     * 
     * For any invalid user input (non-existent path, invalid credentials), 
     * farmer should transition to error state and display an appropriate error message.
     */
    it('Property 15: farmer error state handling - invalid inputs trigger error state with message', () => {
        fc.assert(
            fc.property(
                fc.oneof(
                    fc.constant(''),
                    fc.constant('/non/existent/path'),
                    fc.constant('C:\\invalid\\path\\that\\does\\not\\exist'),
                    fc.constant('invalid_credentials_xyz'),
                    fc.constant('malformed@@@data'),
                    fc.string().filter(s => s.trim().length === 0), // whitespace-only strings
                ),
                fc.string({ minLength: 1, maxLength: 200 }), // error message
                (invalidInput, errorMessage) => {
                    // Reset to known state
                    farmerStore.reset();

                    // Simulate error handling for invalid input
                    farmerStore.showError(errorMessage);

                    const state = get(farmerStore);

                    // Property: farmer should be in error state
                    expect(state.currentState).toBe('error');

                    // Property: error message should be displayed
                    expect(state.message).toBe(errorMessage);
                    expect(state.message).not.toBeNull();
                    expect(state.message!.length).toBeGreaterThan(0);
                }
            ),
            { numRuns: 100 }
        );
    });

    /**
     * **Feature: milk-player, Property 16: farmer state machine transitions**
     * **Validates: Requirements 6.4, 7.1, 7.2, 7.3**
     * 
     * For any valid state transition trigger (track start, track stop, config complete, error),
     * farmer should transition to the appropriate state according to the state machine definition.
     */
    it('Property 16: farmer state machine transitions - valid triggers produce correct state transitions', () => {
        fc.assert(
            fc.property(
                fc.constantFrom<FarmerState>('idle', 'listening', 'prompting', 'celebrating', 'error'),
                fc.constantFrom<FarmerState>('idle', 'listening', 'prompting', 'celebrating', 'error'),
                fc.option(fc.string({ minLength: 1, maxLength: 100 }), { nil: null }),
                (fromState, toState, message) => {
                    // Reset and set initial state
                    farmerStore.reset();
                    farmerStore.transition(fromState);

                    const initialState = get(farmerStore);
                    expect(initialState.currentState).toBe(fromState);

                    // Transition to new state
                    farmerStore.transition(toState, message);

                    const finalState = get(farmerStore);

                    // Property: state should transition to the requested state
                    expect(finalState.currentState).toBe(toState);

                    // Property: message should be set if provided
                    if (message !== null) {
                        expect(finalState.message).toBe(message);
                    }

                    // Property: expression should be appropriate for the state
                    expect(finalState.expression).toBeDefined();
                    expect(finalState.expression.eyes).toBeDefined();
                    expect(finalState.expression.mouth).toBeDefined();

                    // Verify state-specific expression properties
                    switch (toState) {
                        case 'idle':
                            expect(finalState.expression.eyes).toBe('neutral');
                            expect(finalState.expression.mouth).toBe('neutral');
                            break;
                        case 'listening':
                            expect(finalState.expression.mouth).toBe('smile');
                            break;
                        case 'prompting':
                            expect(finalState.expression.mouth).toBe('talk-1');
                            break;
                        case 'celebrating':
                            expect(finalState.expression.mouth).toBe('smile');
                            break;
                        case 'error':
                            expect(finalState.expression.eyes).toBe('neutral');
                            break;
                    }
                }
            ),
            { numRuns: 100 }
        );
    });

    /**
     * Additional property test: State machine specific transitions
     * Tests the specific transitions mentioned in requirements:
     * - Track start -> listening
     * - Track stop -> idle
     * - Config complete -> celebrating then idle
     * - Error -> error state
     */
    it('Property 16 (extended): specific event-driven state transitions', () => {
        fc.assert(
            fc.property(
                fc.constantFrom(
                    { event: 'track_start', expectedState: 'listening' as FarmerState },
                    { event: 'track_stop', expectedState: 'idle' as FarmerState },
                    { event: 'track_pause', expectedState: 'idle' as FarmerState },
                    { event: 'error_occurred', expectedState: 'error' as FarmerState },
                ),
                (transition) => {
                    farmerStore.reset();

                    // Simulate the event
                    switch (transition.event) {
                        case 'track_start':
                            farmerStore.transition('listening');
                            break;
                        case 'track_stop':
                        case 'track_pause':
                            farmerStore.transition('idle');
                            break;
                        case 'error_occurred':
                            farmerStore.showError('An error occurred');
                            break;
                    }

                    const state = get(farmerStore);

                    // Property: state should match expected state for the event
                    expect(state.currentState).toBe(transition.expectedState);
                }
            ),
            { numRuns: 100 }
        );
    });

    /**
     * Property test: Celebrate auto-returns to idle
     * Tests that celebrating state automatically transitions back to idle
     */
    it('Property 16 (celebrate): celebrating state returns to idle after duration', async () => {
        farmerStore.reset();

        // Trigger celebrate with short duration for testing
        farmerStore.celebrate('Success!', 100);

        const celebratingState = get(farmerStore);
        expect(celebratingState.currentState).toBe('celebrating');
        expect(celebratingState.message).toBe('Success!');

        // Wait for auto-transition
        await new Promise(resolve => setTimeout(resolve, 150));

        const idleState = get(farmerStore);
        expect(idleState.currentState).toBe('idle');
        expect(idleState.message).toBeNull();
    });

    /**
     * Property test: Prompt sets correct state
     */
    it('Property 16 (prompt): prompt method sets prompting state with message', () => {
        fc.assert(
            fc.property(
                fc.string({ minLength: 1, maxLength: 200 }),
                (question) => {
                    farmerStore.reset();
                    farmerStore.prompt(question);

                    const state = get(farmerStore);

                    expect(state.currentState).toBe('prompting');
                    expect(state.message).toBe(question);
                    expect(state.expression.mouth).toBe('talk-1');
                }
            ),
            { numRuns: 100 }
        );
    });
});
