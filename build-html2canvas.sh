#!/bin/sh

mkdir -p dist
(cd html2canvas && npm install && npm run build)
cp html2canvas/dist/html2canvas.min.js dist/