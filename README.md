# 🦀 RustShrink (Image Compression Tool)

**RustShrink** is a powerful, high-performance command-line tool for image compression built in Rust. It uses **MozJPEG** bindings to achieve superior file size reduction while maintaining high visual quality.

## ✨ Features

* **Optimized JPEG Compression:** Achieves smaller files than standard encoders at the same quality setting.
* **High Performance:** Built in Rust for speed and memory efficiency.
* **Configurable Quality:** Easily set the output quality level (1-100).

---

## 🚀 Getting Started

### Prerequisites

* [Rust] (stable)

### Build & Run

1.  Clone the repository:
    ```bash
    git clone [Your-Repo-URL]
    cd rust-shrink
    ```
2.  Run the tool (use `--release` for full optimization):
    ```bash
    cargo run --release -- -i <INPUT_FILE> -o <OUTPUT_FILE> -q <QUALITY>
    ```

---

## 📝 Usage Example

Compress `photo.png` to `output.jpg` with 75% quality:

```bash
cargo run --release -- -i photo.png -o output.jpg -q 75