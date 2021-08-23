/*
pub struct SpriteSheet {
    /// Largura dos quadros (mantenha valores positivos).
    width: i32,

    /// Altura dos quadros (mantenha valores positivos).
    height: i32,

    /// Array de quadros de animação.
    frames: vec<Frame>
}

impl SpriteSheet {
    pub fn new(mut width: i32, mut height: i32) -> Self {
        // Arredondadar dimensões de tamanho para valores positivos:
        width  = width.abs();
        height = height.abs();

        return SpriteSheet {
            width: 0,
            height: 0,
            frames: Vec::new()
        };
    }

    pub fn auto(mut width: i32, mut height: i32, mut rows: i32, mut cols: i32) {
        let frames: Vec<Frame> = Vec::new();

        for i in 0..rows {
            for j in 0..cols {
                let frame: Frame = Frame::new(width * j, height * i, width, height);
                frames.push();
            }
        }
    }
}
*/
