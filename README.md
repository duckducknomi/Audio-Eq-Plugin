# Audio EQ Plugin

A simple audio gain plugin built in Rust using [`nih-plug`](https://github.com/robbert-vdh/nih-plug).  
This plugin applies smooth, logarithmically-scaled gain to stereo audio input.

Supports:
- **CLAP**
- **VST3**

---

## ✨ Features

- Stereo audio support
- Smooth gain control (with logarithmic smoothing)
- Compatible with modern plugin hosts like REAPER

---

## 🛠️ Building and Bundling

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
- A supported DAW (e.g. **REAPER**) for testing
- macOS (for `.vst3` and `.clap` bundles)

### Steps

1. **Clone the repository:**

   ```bash
   git clone https://github.com/your-username/audio-eq-plugin.git
   cd audio-eq-plugin
