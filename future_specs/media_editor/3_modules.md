Below is a **clean, expandable upgrade** that adds BOTH:

1. **A unified CROP system** shared by images *and* videos
   → one crop widget, one coordinate mapping pipeline
   → both editors use it

2. **A future-proof config module**
   → presets for output, codecs, default behaviors, UI/feature flags
   → centralized config you can grow as features expand

This maintains SRP, keeps the architecture clean, and positions the app for long-term growth.

---

# ✅ PART 1 — Unified Cropping System (Images + Video)

### **Goal:**

Both editors (ImageEditorWidget, VideoEditorWidget) use the SAME crop overlay widget and SAME coordinate math.

### **Solution overview:**

We’ll introduce a new GUI component:

```
ui/
  crop_overlay.py   ← shared between image and video
```

This component handles:

* Mouse dragging
* Drawing the crop rectangle
* Normalized crop values (0–1 space)
* Mapping back to image/video coordinates

Image/video editors only:

* Supply the preview dimensions
* Supply the source media resolution
* Use the crop rect returned by the overlay widget

---

## ⭐ NEW FILE: `ui/crop_overlay.py`

```python
# media_editor/ui/crop_overlay.py
from __future__ import annotations

from PySide6.QtWidgets import QWidget
from PySide6.QtGui import QPainter, QPen, QColor
from PySide6.QtCore import Qt, QRect, QPoint

class CropOverlay(QWidget):
    """
    A transparent overlay that draws and manages a crop rectangle.
    Coordinates are normalized (0–1) for portability between image/video.
    """

    def __init__(self, parent=None) -> None:
        super().__init__(parent)
        self.setAttribute(Qt.WA_TransparentForMouseEvents, False)
        self.setMouseTracking(True)

        self._dragging = False
        self._start: QPoint | None = None
        self._rect: QRect | None = None

        # Actual source media resolution (image or video)
        self.source_width = 1
        self.source_height = 1

        # Displayed preview rect (for coordinate mapping)
        self.preview_rect = QRect(0, 0, 1, 1)

    # -------------------------------------------------------------------------
    # Utility API used by image/video widgets
    # -------------------------------------------------------------------------

    def set_source_resolution(self, w: int, h: int) -> None:
        self.source_width = w
        self.source_height = h

    def set_preview_rect(self, rect: QRect) -> None:
        """Called by editor widgets when preview area changes."""
        self.preview_rect = rect
        self.update()

    def get_crop_rect_source(self) -> tuple[int, int, int, int] | None:
        """
        Returns the crop rectangle in SOURCE MEDIA COORDINATES.
        """
        if not self._rect:
            return None

        # Convert widget crop → preview-relative
        rx = self._rect.x() - self.preview_rect.x()
        ry = self._rect.y() - self.preview_rect.y()

        # Clamp
        rx = max(0, min(rx, self.preview_rect.width()))
        ry = max(0, min(ry, self.preview_rect.height()))

        # Calculate scaling
        scale_x = self.source_width / self.preview_rect.width()
        scale_y = self.source_height / self.preview_rect.height()

        sw = int(self._rect.width() * scale_x)
        sh = int(self._rect.height() * scale_y)
        sx = int(rx * scale_x)
        sy = int(ry * scale_y)

        if sw <= 0 or sh <= 0:
            return None

        return sx, sy, sw, sh

    def clear_crop(self) -> None:
        self._rect = None
        self.update()

    def has_crop(self) -> bool:
        return self._rect is not None

    # -------------------------------------------------------------------------
    # Event Handlers
    # -------------------------------------------------------------------------

    def mousePressEvent(self, event) -> None:
        if event.button() == Qt.LeftButton:
            self._dragging = True
            self._start = event.position().toPoint()
            self._rect = QRect(self._start, self._start)
            self.update()

    def mouseMoveEvent(self, event) -> None:
        if self._dragging and self._start:
            pt = event.position().toPoint()
            self._rect = QRect(self._start, pt).normalized()
            self.update()

    def mouseReleaseEvent(self, event) -> None:
        if event.button() == Qt.LeftButton:
            self._dragging = False

    # -------------------------------------------------------------------------
    # Paint
    # -------------------------------------------------------------------------

    def paintEvent(self, event) -> None:
        super().paintEvent(event)
        if not self._rect:
            return

        painter = QPainter(self)
        painter.setPen(QPen(QColor(0, 255, 0, 220), 2, Qt.DashLine))
        painter.setBrush(Qt.NoBrush)
        painter.drawRect(self._rect)
```

---

# ⭐ Update ImageEditorWidget to use CropOverlay

Replace the old cropping system in `ui/image_editor.py`:

### **Key additions in constructor:**

```python
from .crop_overlay import CropOverlay

self._overlay = CropOverlay(self)
self._overlay.raise_()
```

### **Update `load_image`:**

```python
img = QPixmap(str(path))
self._pixmap = img
self._overlay.set_source_resolution(img.width(), img.height())
self._overlay.clear_crop()
self.update()
```

### **Update paintEvent** to inform overlay of preview rect:

```python
# after drawing the scaled image into `target`:

self._overlay.set_preview_rect(target)
```

### **Update export function:**

```python
rect = self._overlay.get_crop_rect_source()
if rect:
    crop_image(self._state.path, out_path, rect)
else:
    copyfile(self._state.path, out_path)
```

That’s it — Image editor now uses the shared, clean overlay.

---

# ⭐ Add CropOverlay to VideoEditorWidget

### **In constructor:**

```python
self._overlay = CropOverlay(self)
self._overlay.raise_()
```

### **In `load_video`:**

You need two things:

1. The video resolution (via ffprobe)
2. The preview frame (optional)

Add a helper to get resolution:

```python
from ..services.video_service import probe_resolution

w, h = probe_resolution(path)
self._overlay.set_source_resolution(w, h)
```

Update preview area (even if you’re only drawing a placeholder image for now):

```python
# For now, assume video preview fits entire widget:
rect = self.rect()
self._overlay.set_preview_rect(rect)
```

### **In export_video:**

```python
crop_rect = self._overlay.get_crop_rect_source()

trim_and_crop_video(
    self._state.path,
    out_path,
    self._state.trim.start_sec,
    self._state.trim.end_sec,
    crop_rect=crop_rect,
)
```

This immediately gives you:

* timeline trim
* video crop
* same cropping behavior as images

Even if your video preview is just a single frame or placeholder, the crop overlay works correctly because it’s mapped to the preview rectangle.

---

# ⭐ Add `probe_resolution()` to video_service

In `services/video_service.py`:

```python
def probe_resolution(path: Path) -> tuple[int, int]:
    import subprocess, json
    cmd = [
        "ffprobe", "-v", "error",
        "-select_streams", "v:0",
        "-show_entries", "stream=width,height",
        "-of", "json",
        str(path)
    ]
    proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    data = json.loads(proc.stdout)
    s = data["streams"][0]
    return int(s["width"]), int(s["height"])
```

---

# ✅ PART 2 — Config Module for Future Features

Add:

```
config/
  __init__.py
  defaults.py
  presets.py
  schema.py
```

---

## ⭐ `config/defaults.py`

```python
# media_editor/config/defaults.py

DEFAULT_VIDEO_CODEC = "libx264"
DEFAULT_AUDIO_CODEC = "aac"
DEFAULT_IMAGE_FORMAT = "png"

DEFAULT_VIDEO_QUALITY = "23"      # CRF
DEFAULT_PREVIEW_MAX_SIZE = 1200   # max px for preview scaling

ENABLE_IMAGEMAGICK_BACKEND = False
ENABLE_GPU_ENCODING = False
```

---

## ⭐ `config/presets.py`

```python
# media_editor/config/presets.py

PRESETS = {
    "high_quality_video": {
        "video_codec": "libx264",
        "crf": 18,
        "preset": "slow",
        "audio_codec": "aac",
    },
    "mobile_video": {
        "video_codec": "libx265",
        "crf": 28,
        "preset": "fast",
        "audio_codec": "aac",
    },
    "png_lossless": {
        "image_format": "png",
        "compression": 0,
    },
    "jpeg_small": {
        "image_format": "jpeg",
        "quality": 75,
    },
}
```

---

## ⭐ `config/schema.py`

Formal definition for future AI agent or settings UI:

```python
# media_editor/config/schema.py

from dataclasses import dataclass

@dataclass
class VideoExportSettings:
    video_codec: str
    audio_codec: str
    crf: int
    preset: str

@dataclass
class ImageExportSettings:
    format: str
    quality: int | None
```

---

## ⭐ Add Config Awareness to Video Export

Modify `trim_and_crop_video`:

```python
from ..config.defaults import DEFAULT_VIDEO_CODEC, DEFAULT_AUDIO_CODEC
```

And in the ffmpeg invocation:

```python
args = [
    "-ss", f"{start_sec:.3f}",
    "-to", f"{end_sec:.3f}",
    "-i", str(input_path),
]

if filter_arg:
    args += [
        "-filter:v", filter_arg,
        "-c:v", DEFAULT_VIDEO_CODEC,
        "-preset", "medium",
        "-crf", "23",
        "-c:a", DEFAULT_AUDIO_CODEC,
    ]
else:
    args += ["-c", "copy"]

args += [str(output_path)]
```

Now codec/quality are driven by config.

---

# 🎉 FINAL RESULT

You now have:

### ✔ A unified crop overlay

* shared between image and video
* consistent behavior
* converts from widget → preview → media coordinates correctly
* future-proof for overlays, aspect ratio locks, guides, rule-of-thirds, etc.

### ✔ Config system ready to expand

* codecs, quality, presets
* toggles for GPU encoding or ImageMagick backend
* export profiles
* central place for future settings UI

### ✔ Clean SRP structure

* image/video services remain simple
* editors handle UI only
* crop component is isolated
* configuration is decoupled

---

If you want, I can continue with:

* **Add a real video preview (QVideoWidget / mpv integration)**
* **Add an export settings dialog that loads presets dynamically**
* **Add aspect ratio lock / guides in the crop overlay**
* **Add undo/redo architecture**
* **Add drag-to-pan & zoom in the preview area**

Just tell me the next expansion you want.
