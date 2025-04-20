<p align="center">
  <img src="https://www.rust-lang.org/logos/rust-logo-512x512.png" width="100" alt="Rust logo">
</p>

<h1 align="center">🎯 Silhouette Tracker</h1>

<p align="center">
  A fast and type-safe silhouette detection tool built with Rust.
</p>

---

## 🧠 What It Does

This CLI tool loads an input image, detects high-contrast edges using a Sobel filter, and outputs a new image showing the detected silhouette.

It currently runs on static image files, with plans to support real-time webcam tracking via OpenCV.

---

## ⚙️ Features

- 🦀 Built in 100% safe, modern Rust
- 🖼️ Grayscale & edge detection using `ndarray` + `image`
- 🎛️ Configurable via CLI (input/output path)
- 🧠 Auto-detects output file format from input

---

## 🚀 Usage

```bash
# Use default output name based on input extension
cargo run -- input.jpg
# ➜ output_silhouette.jpg

# Explicit output using long flag
cargo run -- input.png --output silhouette.png

# Use shorthand flag for output
cargo run -- input.webp -o outlined.webp
```

### 🔧 Build the Project

```bash
cargo build --release
```
