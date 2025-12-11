Nice, this is a perfect “small but serious” Python app.

I’ll give you a **future-proof architecture** + **file layout** + **starter code skeletons** so you can grow this into a real editor instead of a throwaway script.

---

## 1. Tech choices

- **GUI:** PySide6 (Qt for Python)
- **Images (preview & crop):** Pillow
- **Video (trim & crop):** FFmpeg via `subprocess` (you install ffmpeg separately)
- **Structure:** small “app” package with SRP modules

ImageMagick can be layered in later as an alternative backend, but for a first pass, Pillow + ffmpeg is enough.

---

## 2. Project layout

Something like:

```text
media_editor/
  pyproject.toml           # or requirements.txt / setup.cfg
  media_editor/
    __init__.py

    main.py                # app entry point

    models/
      __init__.py
      media_state.py       # dataclasses / state containers
      enums.py             # MediaType, OperationType, etc.

    services/
      __init__.py
      image_service.py     # crop/resize/save images (Pillow)
      video_service.py     # trim/crop videos (ffmpeg)
      ffmpeg_service.py    # low-level ffmpeg command builder/runner

    ui/
      __init__.py
      main_window.py       # QMainWindow, menus, routing
      image_editor.py      # widget for image preview + crop rectangle
      video_editor.py      # widget for video preview + timeline trim

    utils/
      __init__.py
      paths.py             # temp paths, safe filenames, etc.
      math_utils.py        # coordinate mapping, aspect ratio helpers
```

You can keep it all in one folder for now and promote to a package later.

---

## 3. Core models

### `models/enums.py`

```python
# media_editor/models/enums.py
from enum import Enum, auto

class MediaType(Enum):
    IMAGE = auto()
    VIDEO = auto()
```

### `models/media_state.py`

```python
# media_editor/models/media_state.py
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional, Tuple
from .enums import MediaType

Rect = Tuple[int, int, int, int]  # x, y, w, h

@dataclass
class CropState:
    rect: Optional[Rect] = None     # current crop rectangle in *source* coords

@dataclass
class TrimState:
    start_sec: float = 0.0
    end_sec: float = 0.0
    duration_sec: float = 0.0

@dataclass
class MediaState:
    path: Optional[Path] = None
    media_type: Optional[MediaType] = None

    crop: CropState = field(default_factory=CropState)
    trim: TrimState = field(default_factory=TrimState)
```

---

## 4. Backend services

### `services/ffmpeg_service.py`

Centralized FFmpeg runner:

```python
# media_editor/services/ffmpeg_service.py
from __future__ import annotations
import subprocess
from pathlib import Path
from typing import List

class FFmpegError(RuntimeError):
    pass

def run_ffmpeg(args: List[str]) -> None:
    cmd = ["ffmpeg", "-y"] + args
    proc = subprocess.run(
        cmd,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True
    )
    if proc.returncode != 0:
        raise FFmpegError(proc.stderr)
```

### `services/video_service.py`

Trim + crop:

```python
# media_editor/services/video_service.py
from __future__ import annotations
from pathlib import Path
from .ffmpeg_service import run_ffmpeg

def probe_duration(input_path: Path) -> float:
    """Use ffprobe to get video duration (seconds)."""
    import subprocess, json

    cmd = [
        "ffprobe", "-v", "error",
        "-select_streams", "v:0",
        "-show_entries", "format=duration",
        "-of", "json",
        str(input_path)
    ]
    proc = subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr)

    data = json.loads(proc.stdout)
    return float(data["format"]["duration"])

def trim_and_crop_video(
    input_path: Path,
    output_path: Path,
    start_sec: float,
    end_sec: float,
    crop_rect: tuple[int, int, int, int] | None = None,
) -> None:
    """Trim to [start_sec, end_sec], optionally crop."""
    filters = []

    if crop_rect:
        x, y, w, h = crop_rect
        filters.append(f"crop={w}:{h}:{x}:{y}")

    filter_arg = None
    if filters:
        filter_arg = ",".join(filters)

    args = [
        "-ss", f"{start_sec:.3f}",
        "-to", f"{end_sec:.3f}",
        "-i", str(input_path),
    ]

    if filter_arg:
        args += ["-filter:v", filter_arg, "-c:v", "libx264", "-c:a", "copy"]
    else:
        # try stream copy when no filters applied
        args += ["-c", "copy"]

    args += [str(output_path)]

    run_ffmpeg(args)
```

### `services/image_service.py`

Pillow handling:

```python
# media_editor/services/image_service.py
from __future__ import annotations
from pathlib import Path
from typing import Tuple
from PIL import Image

Rect = Tuple[int, int, int, int]  # x, y, w, h

def load_image(path: Path) -> Image.Image:
    return Image.open(path).convert("RGBA")

def crop_image(
    input_path: Path,
    output_path: Path,
    rect: Rect,
) -> None:
    x, y, w, h = rect
    with Image.open(input_path) as im:
        box = (x, y, x + w, y + h)
        cropped = im.crop(box)
        cropped.save(output_path)
```

---

## 5. GUI entry point

### `main.py`

```python
# media_editor/main.py
import sys
from PySide6.QtWidgets import QApplication
from .ui.main_window import MainWindow

def main() -> None:
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec())

if __name__ == "__main__":
    main()
```

---

## 6. Main window & routing

### `ui/main_window.py`

- Menu: Open, Save As
- A `QStackedWidget` to swap between image-editor and video-editor views

```python
# media_editor/ui/main_window.py
from __future__ import annotations
from pathlib import Path

from PySide6.QtWidgets import (
    QMainWindow,
    QFileDialog,
    QStackedWidget,
    QMessageBox,
)
from PySide6.QtCore import Qt

from ..models.media_state import MediaState
from ..models.enums import MediaType
from .image_editor import ImageEditorWidget
from .video_editor import VideoEditorWidget
from ..services.video_service import probe_duration

class MainWindow(QMainWindow):
    def __init__(self, parent=None) -> None:
        super().__init__(parent)
        self.setWindowTitle("Simple Media Trimmer")

        self._state = MediaState()

        self._stack = QStackedWidget()
        self._image_editor = ImageEditorWidget(self._state, self)
        self._video_editor = VideoEditorWidget(self._state, self)

        self._stack.addWidget(self._image_editor)
        self._stack.addWidget(self._video_editor)

        self.setCentralWidget(self._stack)

        self._create_menus()

    # --- Menus ----------------------------------------------------------------

    def _create_menus(self) -> None:
        menubar = self.menuBar()

        file_menu = menubar.addMenu("&File")

        open_action = file_menu.addAction("Open…")
        open_action.triggered.connect(self.on_open)

        save_action = file_menu.addAction("Save As…")
        save_action.triggered.connect(self.on_save_as)

    # --- Slots -----------------------------------------------------------------

    def on_open(self) -> None:
        path_str, _ = QFileDialog.getOpenFileName(
            self,
            "Open media",
            "",
            "Media files (*.png *.jpg *.jpeg *.bmp *.gif *.mp4 *.mov *.mkv);;All files (*.*)",
        )
        if not path_str:
            return

        path = Path(path_str)
        self._load_media(path)

    def _load_media(self, path: Path) -> None:
        suffix = path.suffix.lower()
        if suffix in {".png", ".jpg", ".jpeg", ".bmp", ".gif"}:
            media_type = MediaType.IMAGE
        elif suffix in {".mp4", ".mov", ".mkv"}:
            media_type = MediaType.VIDEO
        else:
            QMessageBox.warning(self, "Unsupported", f"Unsupported file type: {suffix}")
            return

        self._state.path = path
        self._state.media_type = media_type

        if media_type is MediaType.IMAGE:
            self._stack.setCurrentWidget(self._image_editor)
            self._image_editor.load_image(path)
        else:
            duration = probe_duration(path)
            self._state.trim.duration_sec = duration
            self._state.trim.end_sec = duration
            self._stack.setCurrentWidget(self._video_editor)
            self._video_editor.load_video(path, duration)

    def on_save_as(self) -> None:
        if not self._state.path or not self._state.media_type:
            QMessageBox.information(self, "No media", "Open a file first.")
            return

        out_str, _ = QFileDialog.getSaveFileName(
            self,
            "Save output",
            "",
            "All files (*.*)",
        )
        if not out_str:
            return

        out_path = Path(out_str)

        try:
            if self._state.media_type is MediaType.IMAGE:
                self._image_editor.export_image(out_path)
            else:
                self._video_editor.export_video(out_path)
        except Exception as e:
            QMessageBox.critical(self, "Error", str(e))
```

---

## 7. Image editor: crop with mouse

### `ui/image_editor.py`

This is a minimal but extendable version:

```python
# media_editor/ui/image_editor.py
from __future__ import annotations

from pathlib import Path
from typing import Optional

from PySide6.QtWidgets import QWidget
from PySide6.QtGui import QPainter, QPixmap, QPen, QColor
from PySide6.QtCore import Qt, QRect, QPoint

from ..models.media_state import MediaState
from ..services.image_service import crop_image

class ImageEditorWidget(QWidget):
    def __init__(self, state: MediaState, parent=None) -> None:
        super().__init__(parent)
        self._state = state
        self._pixmap: Optional[QPixmap] = None

        # crop in widget coordinates
        self._dragging = False
        self._drag_start: Optional[QPoint] = None
        self._current_rect: Optional[QRect] = None

    # --- Public API -----------------------------------------------------------

    def load_image(self, path: Path) -> None:
        self._pixmap = QPixmap(str(path))
        self._reset_crop()
        self.update()

    def export_image(self, out_path: Path) -> None:
        if not self._state.path:
            raise RuntimeError("No image loaded")
        if not self._current_rect:
            # No crop: just copy
            from shutil import copyfile
            copyfile(self._state.path, out_path)
            return

        # Map widget-rect → image-rect
        rect = self._widget_to_image_rect(self._current_rect)
        crop_image(self._state.path, out_path, rect)

    # --- Internal helpers -----------------------------------------------------

    def _reset_crop(self) -> None:
        self._dragging = False
        self._drag_start = None
        self._current_rect = None

    def _widget_to_image_rect(self, rect: QRect) -> tuple[int, int, int, int]:
        """Convert selected rect in widget coords to image coords."""
        if not self._pixmap:
            return (0, 0, 0, 0)

        img_w = self._pixmap.width()
        img_h = self._pixmap.height()
        w_w = self.width()
        w_h = self.height()

        # Fit image to widget while preserving aspect ratio
        scale = min(w_w / img_w, w_h / img_h)
        draw_w = int(img_w * scale)
        draw_h = int(img_h * scale)
        offset_x = (w_w - draw_w) // 2
        offset_y = (w_h - draw_h) // 2

        # Intersection with drawing area
        rx = rect.x() - offset_x
        ry = rect.y() - offset_y

        # Map back to image coords
        x = max(0, int(rx / scale))
        y = max(0, int(ry / scale))
        w = int(rect.width() / scale)
        h = int(rect.height() / scale)

        # Clamp
        x = max(0, min(x, img_w - 1))
        y = max(0, min(y, img_h - 1))
        w = max(1, min(w, img_w - x))
        h = max(1, min(h, img_h - y))

        return (x, y, w, h)

    # --- Events ---------------------------------------------------------------

    def paintEvent(self, event) -> None:
        super().paintEvent(event)
        painter = QPainter(self)

        if self._pixmap:
            img_w = self._pixmap.width()
            img_h = self._pixmap.height()
            w_w = self.width()
            w_h = self.height()

            scale = min(w_w / img_w, w_h / img_h)
            draw_w = int(img_w * scale)
            draw_h = int(img_h * scale)
            offset_x = (w_w - draw_w) // 2
            offset_y = (w_h - draw_h) // 2

            target = QRect(offset_x, offset_y, draw_w, draw_h)
            painter.drawPixmap(target, self._pixmap)

        if self._current_rect:
            pen = QPen(QColor(0, 255, 0), 2, Qt.DashLine)
            painter.setPen(pen)
            painter.drawRect(self._current_rect)

    def mousePressEvent(self, event) -> None:
        if event.button() == Qt.LeftButton:
            self._dragging = True
            self._drag_start = event.position().toPoint()
            self._current_rect = QRect(self._drag_start, self._drag_start)
            self.update()

    def mouseMoveEvent(self, event) -> None:
        if self._dragging and self._drag_start:
            current = event.position().toPoint()
            self._current_rect = QRect(self._drag_start, current).normalized()
            self.update()

    def mouseReleaseEvent(self, event) -> None:
        if event.button() == Qt.LeftButton:
            self._dragging = False
```

That gives you a simple click-and-drag crop box with basic mapping to image coordinates.

---

## 8. Video editor: minimal timeline trim

Full video preview can be added later (with a video widget or just thumbnails). For now, **just trimming on a timeline**.

### `ui/video_editor.py`

```python
# media_editor/ui/video_editor.py
from __future__ import annotations

from pathlib import Path

from PySide6.QtWidgets import (
    QWidget,
    QVBoxLayout,
    QLabel,
    QSlider,
    QHBoxLayout,
)
from PySide6.QtCore import Qt

from ..models.media_state import MediaState
from ..services.video_service import trim_and_crop_video

class VideoEditorWidget(QWidget):
    def __init__(self, state: MediaState, parent=None) -> None:
        super().__init__(parent)
        self._state = state
        self._duration = 0.0

        layout = QVBoxLayout(self)

        self._label = QLabel("Trim: [0.0s – 0.0s]")
        layout.addWidget(self._label)

        sliders_layout = QHBoxLayout()
        layout.addLayout(sliders_layout)

        self._start_slider = QSlider(Qt.Horizontal)
        self._end_slider = QSlider(Qt.Horizontal)

        for s in (self._start_slider, self._end_slider):
            s.setMinimum(0)
            s.valueChanged.connect(self._on_slider_changed)

        sliders_layout.addWidget(self._start_slider)
        sliders_layout.addWidget(self._end_slider)

    def load_video(self, path: Path, duration: float) -> None:
        self._duration = duration
        max_ticks = 1000
        for s in (self._start_slider, self._end_slider):
            s.blockSignals(True)
            s.setMaximum(max_ticks)
            s.blockSignals(False)

        self._start_slider.setValue(0)
        self._end_slider.setValue(max_ticks)

        self._update_state_from_sliders()

    def _on_slider_changed(self) -> None:
        # ensure start <= end
        if self._start_slider.value() > self._end_slider.value():
            self._end_slider.blockSignals(True)
            self._end_slider.setValue(self._start_slider.value())
            self._end_slider.blockSignals(False)

        self._update_state_from_sliders()

    def _update_state_from_sliders(self) -> None:
        max_ticks = max(self._start_slider.maximum(), 1)
        start_frac = self._start_slider.value() / max_ticks
        end_frac = self._end_slider.value() / max_ticks

        self._state.trim.start_sec = start_frac * self._duration
        self._state.trim.end_sec = end_frac * self._duration
        self._state.trim.duration_sec = self._duration

        self._label.setText(
            f"Trim: [{self._state.trim.start_sec:.2f}s – {self._state.trim.end_sec:.2f}s]"
        )

    def export_video(self, out_path: Path) -> None:
        if not self._state.path:
            raise RuntimeError("No video loaded")

        # No crop for now: you can pass a crop_rect later
        trim_and_crop_video(
            self._state.path,
            out_path,
            self._state.trim.start_sec,
            self._state.trim.end_sec,
            crop_rect=None,
        )
```

Later you can add:

- Crop overlay on a preview frame
- A proper video widget for playback
- Keyframe snapping, etc.

---

## 9. How this matches your 3 use cases

1. **Dynamic canvas crop (images/videos)**
   - Images: drag crop box in `ImageEditorWidget`; export uses `crop_image`.
   - Videos: later you can add a crop overlay to `VideoEditorWidget` and pass rect → `trim_and_crop_video()`.

2. **Timeline trim**
   - `VideoEditorWidget` uses two sliders to define `[start_sec, end_sec]` and calls ffmpeg.

3. **Load/save (import/export)**
   - `MainWindow` handles Open / Save As and delegates to the appropriate editor.

---

If you want, next step I can:

- Add **video crop overlay** to match the image behavior (single shared crop logic).
- Or add a **config module** for future features (output presets, codecs, etc.).
