/// `@class` Color
///
/// Representa uma cor RGBA usada na paleta de um bitmap.
pub struct Color {
    /// Canal de cor vermelho.
    pub r: u8,

    /// Canal de cor verde.
    pub g: u8,

    /// Canal de cor azul.
    pub b: u8,

    /// Canal de transparência (não usado).
    pub a: u8
}

impl Color {
    /// `@constructor`
    ///
    /// # Parâmetro(s):
    ///
    /// `r` - Canal de cor vermelho.
    /// `g` - Canal de cor verde.
    /// `b` - Canal de cor azul.
    /// `a` - Canal de transparência (não usado).
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        return Color {
            r: r,
            g: g,
            b: b,
            a: a
        };
    }

    /// Redefine todas as cores através dos mesmos parâmetros do construtor.
    ///
    /// `r` - Canal de cor vermelho.
    /// `g` - Canal de cor verde.
    /// `b` - Canal de cor azul.
    /// `a` - Canal de transparência (não usado).
    pub fn set(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
        self.a = a;
    }

    /// # Retorna:
    ///
    /// O valor da cor em hexadecimal único (RGB).
    pub fn to_hex(&self) -> u32 {
        let r: u32 = self.r.into();
        let g: u32 = self.g.into();
        let b: u32 = self.b.into();

        return b + (g * 0x100) + (r * 0x10000);
    }

    /// Redefine todas as cores através de uma instância pré-existente.
    ///
    /// `r` - Canal de cor vermelho.
    /// `g` - Canal de cor verde.
    /// `b` - Canal de cor azul.
    /// `a` - Canal de transparência (não usado).
    pub fn set_from(&mut self, color: &Color) {
        self.set(color.r, color.g, color.b, color.a);
    }

    /// # Retorna:
    ///
    /// Uma cópia desta instância.
    pub fn clone(&self) -> Color {
        return Color::new(self.r, self.g, self.b, self.a);
    }
}
