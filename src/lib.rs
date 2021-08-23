use std::io::prelude::*;
use std::fs::File;

use std::fs;
use std::env;

mod bitmap;
mod color;
mod frame;
mod palette;

use bitmap::Bitmap;
use color::Color;
use frame::Frame;
use palette::Palette;

