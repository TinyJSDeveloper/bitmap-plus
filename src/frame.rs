pub struct Frame {
    x_start: i32,
    y_start: i32,
    width: i32,
    height: i32
}

impl Frame {
    pub fn new(x_start: i32, y_start: i32, width: i32, height: i32) -> Self {
        return Frame {
            x_start: x_start,
            y_start: y_start,
            width: width,
            height: height
        };
    }

    pub fn get_x_start(&self) -> i32 {
        return self.x_start;
    }

    pub fn get_y_start(&self) -> i32 {
        return self.y_start;
    }

    pub fn get_width(&self) -> i32 {
        return self.width;
    }

    pub fn get_height(&self) -> i32 {
        return self.height;
    }
}
