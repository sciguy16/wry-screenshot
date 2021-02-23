# wry-screenshot
Example utility to take webpage screenshots with [Wry](https://github.com/tauri-apps/wry).

Currently uses [html2canvas](https://github.com/niklasvh/html2canvas) from within the webview, see [tauri-apps/wry#70](https://github.com/tauri-apps/wry/issues/70).

## Usage
```bash
git clone https://github.com/sciguy16/wry-screenshot
cargo run -- http://example.com
# Captured image is 'image.png'
```

## Setup
This repo ships with a compiled version of html2canvas.
To rebuild it, ensure that you have checked out the html2canvas submodule and build it with npm:
```bash
git clone --recursive https://github.com/sciguy16/wry-screenshot
./build-html2canvas.sh
```