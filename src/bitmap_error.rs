#[derive(Debug)]
struct BitmapError {
    // ...
}

impl Error for BitmapError {
    // ...
}

impl fmt::Display for BitmapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
