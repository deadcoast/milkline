/**
 * Coordinate transformation utilities for media editor
 *
 * Handles mapping between three coordinate systems:
 * 1. Widget Coordinates: Mouse position in the UI widget (pixels)
 * 2. Preview Coordinates: Position relative to the displayed preview area (pixels)
 * 3. Source Coordinates: Position in the original media file (pixels)
 */

export interface Point {
  x: number;
  y: number;
}

export interface Rectangle {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface PreviewBounds {
  offsetX: number;
  offsetY: number;
  width: number;
  height: number;
}

/**
 * Convert widget coordinates to preview coordinates
 *
 * @param widgetX - X coordinate in widget space
 * @param widgetY - Y coordinate in widget space
 * @param previewBounds - The bounds of the preview area within the widget
 * @returns Point in preview coordinate space
 */
export function widgetToPreviewCoords(
  widgetX: number,
  widgetY: number,
  previewBounds: PreviewBounds,
): Point {
  return {
    x: widgetX - previewBounds.offsetX,
    y: widgetY - previewBounds.offsetY,
  };
}

/**
 * Convert preview coordinates to source coordinates
 *
 * @param previewX - X coordinate in preview space
 * @param previewY - Y coordinate in preview space
 * @param previewWidth - Width of the preview area
 * @param previewHeight - Height of the preview area
 * @param sourceWidth - Width of the source media
 * @param sourceHeight - Height of the source media
 * @returns Point in source coordinate space, clamped to valid bounds
 */
export function previewToSourceCoords(
  previewX: number,
  previewY: number,
  previewWidth: number,
  previewHeight: number,
  sourceWidth: number,
  sourceHeight: number,
): Point {
  const scaleX = calculateScaleFactor(previewWidth, sourceWidth);
  const scaleY = calculateScaleFactor(previewHeight, sourceHeight);

  const x = Math.round(previewX / scaleX);
  const y = Math.round(previewY / scaleY);

  // Clamp to valid source bounds to handle rounding edge cases
  return {
    x: Math.max(0, Math.min(x, sourceWidth - 1)),
    y: Math.max(0, Math.min(y, sourceHeight - 1)),
  };
}

/**
 * Clamp coordinates to valid source bounds
 *
 * @param x - X coordinate to clamp
 * @param y - Y coordinate to clamp
 * @param width - Width to clamp
 * @param height - Height to clamp
 * @param sourceWidth - Maximum width (source media width)
 * @param sourceHeight - Maximum height (source media height)
 * @returns Clamped rectangle
 */
export function clampToSourceBounds(
  x: number,
  y: number,
  width: number,
  height: number,
  sourceWidth: number,
  sourceHeight: number,
): Rectangle {
  // Clamp position to valid range
  const clampedX = Math.max(0, Math.min(x, sourceWidth - 1));
  const clampedY = Math.max(0, Math.min(y, sourceHeight - 1));

  // Clamp dimensions to fit within bounds
  const maxWidth = sourceWidth - clampedX;
  const maxHeight = sourceHeight - clampedY;
  const clampedWidth = Math.max(1, Math.min(width, maxWidth));
  const clampedHeight = Math.max(1, Math.min(height, maxHeight));

  return {
    x: clampedX,
    y: clampedY,
    width: clampedWidth,
    height: clampedHeight,
  };
}

/**
 * Calculate scale factor between preview and source dimensions
 *
 * @param previewDimension - Preview dimension (width or height)
 * @param sourceDimension - Source dimension (width or height)
 * @returns Scale factor (preview / source)
 */
export function calculateScaleFactor(
  previewDimension: number,
  sourceDimension: number,
): number {
  if (sourceDimension === 0) {
    return 1;
  }
  return previewDimension / sourceDimension;
}

/**
 * Convert a rectangle from preview to source coordinates
 *
 * @param rect - Rectangle in preview coordinates
 * @param previewWidth - Width of the preview area
 * @param previewHeight - Height of the preview area
 * @param sourceWidth - Width of the source media
 * @param sourceHeight - Height of the source media
 * @returns Rectangle in source coordinates, clamped to valid bounds
 */
export function previewRectToSourceRect(
  rect: Rectangle,
  previewWidth: number,
  previewHeight: number,
  sourceWidth: number,
  sourceHeight: number,
): Rectangle {
  const topLeft = previewToSourceCoords(
    rect.x,
    rect.y,
    previewWidth,
    previewHeight,
    sourceWidth,
    sourceHeight,
  );

  const bottomRight = previewToSourceCoords(
    rect.x + rect.width,
    rect.y + rect.height,
    previewWidth,
    previewHeight,
    sourceWidth,
    sourceHeight,
  );

  const width = bottomRight.x - topLeft.x;
  const height = bottomRight.y - topLeft.y;

  return clampToSourceBounds(
    topLeft.x,
    topLeft.y,
    width,
    height,
    sourceWidth,
    sourceHeight,
  );
}

/**
 * Convert a point from source to preview coordinates
 *
 * @param sourceX - X coordinate in source space
 * @param sourceY - Y coordinate in source space
 * @param previewWidth - Width of the preview area
 * @param previewHeight - Height of the preview area
 * @param sourceWidth - Width of the source media
 * @param sourceHeight - Height of the source media
 * @returns Point in preview coordinate space
 */
export function sourceToPreviewCoords(
  sourceX: number,
  sourceY: number,
  previewWidth: number,
  previewHeight: number,
  sourceWidth: number,
  sourceHeight: number,
): Point {
  const scaleX = calculateScaleFactor(previewWidth, sourceWidth);
  const scaleY = calculateScaleFactor(previewHeight, sourceHeight);

  return {
    x: Math.round(sourceX * scaleX),
    y: Math.round(sourceY * scaleY),
  };
}
