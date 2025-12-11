import { describe, it, expect } from "vitest";
import * as fc from "fast-check";

/**
 * Extract the getMediaTypeFromExtension logic for testing
 * This is the same logic used in MediaEditorWindow.svelte
 */
function getMediaTypeFromExtension(filePath: string): "image" | "video" | null {
  const extension = filePath.split(".").pop()?.toLowerCase();

  const imageExtensions = ["png", "jpg", "jpeg", "bmp", "gif"];
  const videoExtensions = ["mp4", "mov", "mkv"];

  if (extension && imageExtensions.includes(extension)) {
    return "image";
  } else if (extension && videoExtensions.includes(extension)) {
    return "video";
  }

  return null;
}

describe("MediaEditorWindow property-based tests", () => {
  // Feature: media-editor, Property 3: Media type determines editor routing
  // Validates: Requirements 4.2, 4.3
  describe("Property 3: Media type determines editor routing", () => {
    it("should route all image extensions to image editor", () => {
      fc.assert(
        fc.property(
          // Generate random file paths with image extensions
          fc.constantFrom("png", "jpg", "jpeg", "bmp", "gif"),
          fc.string({ minLength: 1, maxLength: 50 }),
          (extension, filename) => {
            const filePath = `${filename}.${extension}`;
            const mediaType = getMediaTypeFromExtension(filePath);

            // All image extensions should route to image editor
            expect(mediaType).toBe("image");
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should route all video extensions to video editor", () => {
      fc.assert(
        fc.property(
          // Generate random file paths with video extensions
          fc.constantFrom("mp4", "mov", "mkv"),
          fc.string({ minLength: 1, maxLength: 50 }),
          (extension, filename) => {
            const filePath = `${filename}.${extension}`;
            const mediaType = getMediaTypeFromExtension(filePath);

            // All video extensions should route to video editor
            expect(mediaType).toBe("video");
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should reject unsupported extensions", () => {
      fc.assert(
        fc.property(
          // Generate random unsupported extensions
          fc.string({ minLength: 1, maxLength: 10 }).filter((ext) => {
            const lower = ext.toLowerCase();
            return ![
              "png",
              "jpg",
              "jpeg",
              "bmp",
              "gif",
              "mp4",
              "mov",
              "mkv",
            ].includes(lower);
          }),
          fc.string({ minLength: 1, maxLength: 50 }),
          (extension, filename) => {
            const filePath = `${filename}.${extension}`;
            const mediaType = getMediaTypeFromExtension(filePath);

            // Unsupported extensions should return null
            expect(mediaType).toBeNull();
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should handle case-insensitive extensions", () => {
      fc.assert(
        fc.property(
          // Generate extensions with random casing
          fc.constantFrom(
            "png",
            "jpg",
            "jpeg",
            "bmp",
            "gif",
            "mp4",
            "mov",
            "mkv",
          ),
          fc.string({ minLength: 1, maxLength: 50 }),
          (extension, filename) => {
            // Create variations with different casing
            const upperExt = extension.toUpperCase();
            const mixedExt = extension
              .split("")
              .map((c, i) => (i % 2 === 0 ? c.toUpperCase() : c.toLowerCase()))
              .join("");

            const lowerPath = `${filename}.${extension}`;
            const upperPath = `${filename}.${upperExt}`;
            const mixedPath = `${filename}.${mixedExt}`;

            const lowerType = getMediaTypeFromExtension(lowerPath);
            const upperType = getMediaTypeFromExtension(upperPath);
            const mixedType = getMediaTypeFromExtension(mixedPath);

            // All variations should produce the same result
            expect(lowerType).toBe(upperType);
            expect(lowerType).toBe(mixedType);
            expect(lowerType).not.toBeNull();
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should handle file paths with multiple dots", () => {
      fc.assert(
        fc.property(
          fc.constantFrom(
            "png",
            "jpg",
            "jpeg",
            "bmp",
            "gif",
            "mp4",
            "mov",
            "mkv",
          ),
          fc.array(fc.string({ minLength: 1, maxLength: 20 }), {
            minLength: 1,
            maxLength: 5,
          }),
          (extension, pathParts) => {
            // Create path with multiple dots
            const filePath = pathParts.join(".") + "." + extension;
            const mediaType = getMediaTypeFromExtension(filePath);

            // Should still correctly identify the extension
            const imageExtensions = ["png", "jpg", "jpeg", "bmp", "gif"];
            const videoExtensions = ["mp4", "mov", "mkv"];

            if (imageExtensions.includes(extension)) {
              expect(mediaType).toBe("image");
            } else if (videoExtensions.includes(extension)) {
              expect(mediaType).toBe("video");
            }
          },
        ),
        { numRuns: 100 },
      );
    });

    it("should handle paths with directory separators", () => {
      fc.assert(
        fc.property(
          fc.constantFrom(
            "png",
            "jpg",
            "jpeg",
            "bmp",
            "gif",
            "mp4",
            "mov",
            "mkv",
          ),
          fc.array(fc.string({ minLength: 1, maxLength: 20 }), {
            minLength: 1,
            maxLength: 3,
          }),
          fc.string({ minLength: 1, maxLength: 20 }),
          (extension, directories, filename) => {
            // Create path with directory separators
            const filePath =
              directories.join("/") + "/" + filename + "." + extension;
            const mediaType = getMediaTypeFromExtension(filePath);

            // Should still correctly identify the extension
            const imageExtensions = ["png", "jpg", "jpeg", "bmp", "gif"];
            const videoExtensions = ["mp4", "mov", "mkv"];

            if (imageExtensions.includes(extension)) {
              expect(mediaType).toBe("image");
            } else if (videoExtensions.includes(extension)) {
              expect(mediaType).toBe("video");
            }
          },
        ),
        { numRuns: 100 },
      );
    });
  });
});
