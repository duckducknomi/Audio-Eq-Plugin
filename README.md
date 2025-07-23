# 🎚️ Audio EQ Plugin

A simple parametric EQ audio plugin built in Rust using [`nih-plug`](https://github.com/robbert-vdh/nih-plug).  
Exposes Frequency, Gain, and Q controls for shaping your sound. Built as a VST3 and CLAP plugin.  
Compatible with macOS and tested with REAPER.

---

## 🔧 Requirements

- Rust toolchain (latest stable)
- Git
- macOS (tested)
- REAPER (or another DAW that supports VST3 or CLAP)
- Xcode (for plugin code signing on macOS)

---

## 🚀 Development Workflow

### 1. Clone the repo

```bash
git clone https://github.com/YOUR_USERNAME/audio-eq-plugin.git
cd audio-eq-plugin
```

### 2. Build and install the plugin

Use the provided script:

```bash
./dev.sh
```

This script will:

- ✅ Run `cargo check`
- ✅ Build the plugin bundles via `cargo xtask`
- ✅ Copy the resulting `.vst3` and `.clap` files into system plugin directories:

| Format | Destination Path |
|--------|------------------|
| VST3   | `~/Library/Audio/Plug-Ins/VST3` |
| CLAP   | `~/Library/Audio/Plug-Ins/CLAP` |

---

## 🧪 Testing in REAPER (macOS)

### ✅ To test new changes without restarting REAPER:

1. Make code changes
2. Run the build script: `./dev.sh`
3. In REAPER:
   - Remove the plugin from any open FX chains
   - Open the FX browser and **click "Re-scan"** at the bottom
   - Re-add the plugin to the track

> This usually picks up the latest build without restarting REAPER.

### 🔁 If the above doesn't work:

Fully quit and relaunch REAPER after building. This guarantees loading the latest version.

---

## 🗂️ Project Structure

```
audio-eq-plugin/
├── src/
│   └── lib.rs           # Main plugin logic (parameters, audio processing)
├── xtask/               # Custom build task used to bundle the plugin
├── target/              # Compiled plugin output
├── bundler.toml         # Plugin metadata for bundling
├── dev.sh               # Build & install helper script
├── Cargo.toml           # Rust project configuration
└── README.md
```

---

## 🎛️ Plugin Parameters

This EQ exposes three tweakable controls:

| Parameter | Range             | Description              |
|-----------|------------------|--------------------------|
| Frequency | 20 Hz – 20 kHz   | Center frequency         |
| Gain      | -24 dB to +24 dB | Boost or cut level       |
| Q         | 0.1 to 10.0      | Filter resonance/bandwidth |

---

## 🔌 Supported Plugin Formats

- ✅ VST3 (scanned and tested in REAPER)
- ✅ CLAP (modern, open plugin format)

---

## 🔒 macOS Plugin Signing

macOS may block new plugins from running. You can fix this by:

1. Going to **System Settings → Privacy & Security**
2. Clicking “Allow Anyway” for the blocked plugin
3. Optionally, you can code sign the plugin yourself

---

## 🦆 About
 
Inspired by analog workflow EQs like the SSL channel strip.  
This plugin focuses on sound shaping rather than surgical correction.

Future plans include:

- ⏺️ Drive knob for harmonic coloration
- 🎛️ Band toggles for intuitive layout
- 🎚️ Custom filters like tilt or mid-bell
- 🖼️ Fully custom GUI

