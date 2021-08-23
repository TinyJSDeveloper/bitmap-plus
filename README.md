# bitmap-plus
Just an ordinary library for bitmaps in Rust.

## Why?
Although there are a lot of bitmap libraries already, I couldn't find one which provided me a way to manipulate images straight out of the box. You can only change a single pixel, then crop the image, but that's it. I also just wanted something that didn't require me to have any dependencies.

This was mostly done as a way to learn how to read and write *BMP*s and it's not intended to replace existing ones. That said, if you need something which is "production ready", you may consider using a more reliable crate instead.

## Supported formats
It can only support uncompressed 256-color bitmap (*\*.bmp*) files, and size must be power of 2 (ex: *16x16*, *800x600*, *etc*).

## Features
* Read and write bitmap (*\*.bmp*) files;
* Set pixels;
* Draw lines/rectangles;
* Draw bitmaps on top of other bitmaps;
* Limited scaling support;
* Alpha masks and blitting;
* Can be used alongside [minifb](https://crates.io/crates/minifb) to preview how the image is being changed;

## Planned features
* Spritesheet support;
* Bitmap font support;
