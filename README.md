# 🎛️ Audio EQ Plugin

A simple VST3/CLAP gain plugin built with [`nih-plug`](https://github.com/robbert-vdh/nih-plug), written in Rust.

This project serves as a boilerplate to get started developing audio plugins using Rust. The included plugin applies linear gain with dB display, built using NIH-plug’s utilities and smoothed parameter handling.

---

## 🛠️ Features

- ✅ VST3 and CLAP plugin formats
- ✅ Stereo I/O
- ✅ Adjustable gain with dB scale
- ✅ Built in Rust using [`nih-plug`](https://github.com/robbert-vdh/nih-plug)

---

## 📦 Building the Plugin

Make sure you have Rust installed (`rustup.rs`), then run:

```bash
cargo run --package xtask -- bundle audio-eq-plugin
```

This will produce:

```
target/bundled/audio-eq-plugin.vst3
target/bundled/audio-eq-plugin.clap
```

---

## 🔗 Linking to REAPER (or other DAWs)

### ✅ Create Symlinks to Plugin Folders

To avoid manually copying every time you rebuild, create a symlink:

#### VST3 (macOS):

```bash
mkdir -p ~/Library/Audio/Plug-Ins/VST3
ln -sfn "$(pwd)/target/bundled/audio-eq-plugin.vst3" ~/Library/Audio/Plug-Ins/VST3/
```

#### CLAP (macOS):

```bash
mkdir -p ~/Library/Audio/Plug-Ins/CLAP
ln -sfn "$(pwd)/target/bundled/audio-eq-plugin.clap" ~/Library/Audio/Plug-Ins/CLAP/
```

> Use `-sfn` to force-update the symlink when rebuilding.

#### 🔄 To remove symlinks later:

```bash
rm ~/Library/Audio/Plug-Ins/VST3/audio-eq-plugin.vst3
rm ~/Library/Audio/Plug-Ins/CLAP/audio-eq-plugin.clap
```

---

## 🎚️ Using in REAPER

1. Open **REAPER**
2. Add a new track
3. Click the **FX** button
4. Search for: `Audio EQ Plugin`
5. Insert it → Adjust the **Gain** slider 🎧

You should see something like this:

> ![Plugin in REAPER](screenshot.png)

---

## 🧠 Credit

Built using [`nih-plug`](https://github.com/robbert-vdh/nih-plug) by [@robbert-vdh](https://github.com/robbert-vdh)

---

## 📄 License

MIT
