# wry-screenshot
Example utility to take webpage screenshots with [Wry](https://github.com/tauri-apps/wry).

Currently uses [html2canvas](https://github.com/niklasvh/html2canvas) from within the webview, see [tauri-apps/wry#70](https://github.com/tauri-apps/wry/issues/70).

## Usage
```bash
git clone https://github.com/sciguy16/wry-screenshot
cargo run -- http://example.com
# Captured image is 'image.png'
```

## Bugs
* Doesn't work for all websites - e.g. some result in `Unhandled Promise Rejection: SecurityError: The operation is insecure.` errors
* Invisible windows seem to ignore the `width` and `height` attributes, resulting in minimum-width images

## Setup
This repo ships with a compiled version of html2canvas.
To rebuild it, ensure that you have checked out the html2canvas submodule and build it with npm:
```bash
git clone --recursive https://github.com/sciguy16/wry-screenshot
./build-html2canvas.sh
```