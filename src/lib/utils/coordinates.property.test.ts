import { describe, it, expect } from 'vitest';
import * as fc from 'fast-check';
import {
    previewToSourceCoords,
    clampToSourceBounds,
    previewRectToSourceRect,
    sourceToPreviewCoords,
    type Rectangle,
} from './coordinates';

describe('coordinates property-based tests', () => {
    // Feature: media-editor, Property 1: Crop coordinate mapping preserves bounds
    // Validates: Requirements 1.4, 2.4
    describe('Property 1: Crop coordinate mapping preserves bounds', () => {
        it('should always produce coordinates within source bounds', () => {
            fc.assert(
                fc.property(
                    // Generate random preview coordinates
                    fc.integer({ min: 0, max: 10000 }), // previewX
                    fc.integer({ min: 0, max: 10000 }), // previewY
                    // Generate random preview dimensions (must be positive)
                    fc.integer({ min: 1, max: 5000 }), // previewWidth
                    fc.integer({ min: 1, max: 5000 }), // previewHeight
                    // Generate random source dimensions (must be positive)
                    fc.integer({ min: 1, max: 5000 }), // sourceWidth
                    fc.integer({ min: 1, max: 5000 }), // sourceHeight
                    (previewX, previewY, previewWidth, previewHeight, sourceWidth, sourceHeight) => {
                        const result = previewToSourceCoords(
                            previewX,
                            previewY,
                            previewWidth,
                            previewHeight,
                            sourceWidth,
                            sourceHeight
                        );

                        // Coordinates should be within valid source bounds
                        expect(result.x).toBeGreaterThanOrEqual(0);
                        expect(result.y).toBeGreaterThanOrEqual(0);
                        expect(result.x).toBeLessThan(sourceWidth);
                        expect(result.y).toBeLessThan(sourceHeight);
                    }
                ),
                { numRuns: 100 }
            );
        });

        it('should produce rectangles with positive dimensions within source bounds', () => {
            fc.assert(
                fc.property(
                    // Generate random rectangle in preview space
                    fc.integer({ min: 0, max: 5000 }), // x
                    fc.integer({ min: 0, max: 5000 }), // y
                    fc.integer({ min: 1, max: 5000 }), // width
                    fc.integer({ min: 1, max: 5000 }), // height
                    // Generate random preview dimensions
                    fc.integer({ min: 1, max: 5000 }), // previewWidth
                    fc.integer({ min: 1, max: 5000 }), // previewHeight
                    // Generate random source dimensions
                    fc.integer({ min: 1, max: 5000 }), // sourceWidth
                    fc.integer({ min: 1, max: 5000 }), // sourceHeight
                    (x, y, width, height, previewWidth, previewHeight, sourceWidth, sourceHeight) => {
                        const rect: Rectangle = { x, y, width, height };
                        const result = previewRectToSourceRect(
                            rect,
                            previewWidth,
                            previewHeight,
                            sourceWidth,
                            sourceHeight
                        );

                        // All coordinates should be within valid source bounds
                        expect(result.x).toBeGreaterThanOrEqual(0);
                        expect(result.y).toBeGreaterThanOrEqual(0);
                        expect(result.x).toBeLessThan(sourceWidth);
                        expect(result.y).toBeLessThan(sourceHeight);

                        // Dimensions should be positive
                        expect(result.width).toBeGreaterThan(0);
                        expect(result.height).toBeGreaterThan(0);

                        // Rectangle should not extend beyond source bounds
                        expect(result.x + result.width).toBeLessThanOrEqual(sourceWidth);
                        expect(result.y + result.height).toBeLessThanOrEqual(sourceHeight);
                    }
                ),
                { numRuns: 100 }
            );
        });

        it('should clamp any coordinates to valid source bounds', () => {
            fc.assert(
                fc.property(
                    // Generate potentially invalid coordinates (including negative)
                    fc.integer({ min: -1000, max: 10000 }), // x
                    fc.integer({ min: -1000, max: 10000 }), // y
                    fc.integer({ min: -100, max: 10000 }), // width
                    fc.integer({ min: -100, max: 10000 }), // height
                    // Generate random source dimensions
                    fc.integer({ min: 1, max: 5000 }), // sourceWidth
                    fc.integer({ min: 1, max: 5000 }), // sourceHeight
                    (x, y, width, height, sourceWidth, sourceHeight) => {
                        const result = clampToSourceBounds(
                            x,
                            y,
                            width,
                            height,
                            sourceWidth,
                            sourceHeight
                        );

                        // All coordinates should be within valid source bounds
                        expect(result.x).toBeGreaterThanOrEqual(0);
                        expect(result.y).toBeGreaterThanOrEqual(0);
                        expect(result.x).toBeLessThan(sourceWidth);
                        expect(result.y).toBeLessThan(sourceHeight);

                        // Dimensions should be positive (minimum 1)
                        expect(result.width).toBeGreaterThanOrEqual(1);
                        expect(result.height).toBeGreaterThanOrEqual(1);

                        // Rectangle should not extend beyond source bounds
                        expect(result.x + result.width).toBeLessThanOrEqual(sourceWidth);
                        expect(result.y + result.height).toBeLessThanOrEqual(sourceHeight);
                    }
                ),
                { numRuns: 100 }
            );
        });
    });

    // Feature: media-editor, Property 8: Coordinate scaling is invertible
    // Validates: Requirements 1.4, 2.4, 6.3
    describe('Property 8: Coordinate scaling is invertible', () => {
        it('should round-trip from source to preview and back with bounded error', () => {
            fc.assert(
                fc.property(
                    // Generate random source dimensions
                    fc.integer({ min: 10, max: 5000 }), // sourceWidth
                    fc.integer({ min: 10, max: 5000 }), // sourceHeight
                    // Generate random preview dimensions
                    fc.integer({ min: 10, max: 5000 }), // previewWidth
                    fc.integer({ min: 10, max: 5000 }), // previewHeight
                    // Generate source coordinates within bounds
                    (sourceWidth, sourceHeight, previewWidth, previewHeight) => {
                        fc.assert(
                            fc.property(
                                fc.integer({ min: 0, max: sourceWidth - 1 }), // sourceX
                                fc.integer({ min: 0, max: sourceHeight - 1 }), // sourceY
                                (sourceX, sourceY) => {
                                    // Convert source -> preview
                                    const preview = sourceToPreviewCoords(
                                        sourceX,
                                        sourceY,
                                        previewWidth,
                                        previewHeight,
                                        sourceWidth,
                                        sourceHeight
                                    );

                                    // Convert preview -> source
                                    const roundTrip = previewToSourceCoords(
                                        preview.x,
                                        preview.y,
                                        previewWidth,
                                        previewHeight,
                                        sourceWidth,
                                        sourceHeight
                                    );

                                    // Calculate scale-dependent tolerance
                                    // Larger scale differences can accumulate more rounding error
                                    const scaleX = previewWidth / sourceWidth;
                                    const scaleY = previewHeight / sourceHeight;
                                    const maxScale = Math.max(scaleX, scaleY, 1 / scaleX, 1 / scaleY);
                                    
                                    // Tolerance grows with scale ratio, minimum 1 pixel
                                    const tolerance = Math.max(1, Math.ceil(maxScale / 2));

                                    const deltaX = Math.abs(roundTrip.x - sourceX);
                                    const deltaY = Math.abs(roundTrip.y - sourceY);

                                    expect(deltaX).toBeLessThanOrEqual(tolerance);
                                    expect(deltaY).toBeLessThanOrEqual(tolerance);
                                }
                            ),
                            { numRuns: 10 }
                        );
                    }
                ),
                { numRuns: 10 }
            );
        });

        it('should maintain relative positions during round-trip', () => {
            fc.assert(
                fc.property(
                    // Generate two points in source space
                    fc.integer({ min: 0, max: 4998 }), // x1
                    fc.integer({ min: 0, max: 4998 }), // y1
                    fc.integer({ min: 1, max: 100 }), // dx (offset from first point)
                    fc.integer({ min: 1, max: 100 }), // dy
                    // Generate dimensions
                    fc.integer({ min: 1, max: 5000 }), // previewWidth
                    fc.integer({ min: 1, max: 5000 }), // previewHeight
                    fc.integer({ min: 1, max: 5000 }), // sourceWidth
                    fc.integer({ min: 1, max: 5000 }), // sourceHeight
                    (x1, y1, dx, dy, previewWidth, previewHeight, sourceWidth, sourceHeight) => {
                        const x2 = x1 + dx;
                        const y2 = y1 + dy;

                        // Ensure both points are within bounds
                        const validX1 = Math.min(x1, sourceWidth - 1);
                        const validY1 = Math.min(y1, sourceHeight - 1);
                        const validX2 = Math.min(x2, sourceWidth - 1);
                        const validY2 = Math.min(y2, sourceHeight - 1);

                        // If points are the same after clamping, skip
                        if (validX1 === validX2 && validY1 === validY2) {
                            return;
                        }

                        // Convert both points to preview
                        const preview1 = sourceToPreviewCoords(
                            validX1,
                            validY1,
                            previewWidth,
                            previewHeight,
                            sourceWidth,
                            sourceHeight
                        );

                        const preview2 = sourceToPreviewCoords(
                            validX2,
                            validY2,
                            previewWidth,
                            previewHeight,
                            sourceWidth,
                            sourceHeight
                        );

                        // Convert back to source
                        const roundTrip1 = previewToSourceCoords(
                            preview1.x,
                            preview1.y,
                            previewWidth,
                            previewHeight,
                            sourceWidth,
                            sourceHeight
                        );

                        const roundTrip2 = previewToSourceCoords(
                            preview2.x,
                            preview2.y,
                            previewWidth,
                            previewHeight,
                            sourceWidth,
                            sourceHeight
                        );

                        // If original x2 > x1, then roundTrip2.x should be >= roundTrip1.x
                        if (validX2 > validX1) {
                            expect(roundTrip2.x).toBeGreaterThanOrEqual(roundTrip1.x - 1);
                        }

                        // If original y2 > y1, then roundTrip2.y should be >= roundTrip1.y
                        if (validY2 > validY1) {
                            expect(roundTrip2.y).toBeGreaterThanOrEqual(roundTrip1.y - 1);
                        }
                    }
                ),
                { numRuns: 100 }
            );
        });
    });
});
