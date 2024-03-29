use crate::Color;

/// `@class` Palette
///
/// Representa uma paleta de cores usada por um bitmap.
pub struct Palette {
    /// `@private`
    /// Array de cores da paleta.
    colors: Vec<Color>,

    /// `@private`
    /// Cor padrão (obtida através de um índice de cor inválido).
    default_color: Color
}

impl Palette {
    /// `@constructor`
    ///
    /// # Parâmetro(s):
    ///
    /// `colors` - Array de cores da paleta.
    pub fn new(colors: Vec<Color>) -> Self {
        return Palette {
            colors: colors,
            default_color: Color::new(0, 0, 0, 0)
        };
    }

    /// # Retorna:
    ///
    /// O número total de cores presentes na paleta.
    pub fn len(&self) -> usize {
        return self.colors.len();
    }

    /// Obtém uma cor de um dos índices da paleta.
    ///
    /// # Parâmetro(s):
    ///
    /// `index` - Índice de cor da paleta.
    ///
    /// # Retorna:
    ///
    /// Uma referência do objeto de cor da paleta. Quando o índice passado, não
    /// existe, uma referência da cor padrão é passada no lugar.
    pub fn get_color(&self, index: usize) -> &Color {
        // Retornar o índice de cor, se existir...
        if index < self.colors.len() {
            return &self.colors[index];
        }

        // ...ou a cor padrão, caso o índice seja inválido:
        return &self.default_color;
    }

    /// # Retorna:
    ///
    /// Uma cópia desta instância.
    pub fn clone(&self) -> Palette {
        // Array de cores para a instância clonada.
        let mut colors: Vec<Color> = Vec::new();

        // Copiar todas as cores para a nova instância...
        for i in 0..self.colors.len() {
            colors.push(Color::clone(self.get_color(i)));
        }

        return Palette::new(colors);
    }

    /// # Retorna:
    ///
    /// Uma paleta de cores padrão. Útil quando se está criando uma nova imagem
    /// dinamicamente e não se tem uma paleta disponível para uso imediato.
    ///
    /// Esta é uma paleta VGA de 256 cores.
    pub fn default() -> Palette {
        return Palette::new(vec![
            Color::new(0x00, 0x00, 0x00, 0x00),
            Color::new(0x00, 0x00, 0xAA, 0x00),
            Color::new(0x00, 0xAA, 0x00, 0x00),
            Color::new(0x00, 0xAA, 0xAA, 0x00),
            Color::new(0xAA, 0x00, 0x00, 0x00),
            Color::new(0xAA, 0x00, 0xAA, 0x00),
            Color::new(0xAA, 0x55, 0x00, 0x00),
            Color::new(0xAA, 0xAA, 0xAA, 0x00),
            Color::new(0x55, 0x55, 0x55, 0x00),
            Color::new(0x55, 0x55, 0xFF, 0x00),
            Color::new(0x55, 0xFF, 0x55, 0x00),
            Color::new(0x55, 0xFF, 0xFF, 0x00),
            Color::new(0xFF, 0x55, 0x55, 0x00),
            Color::new(0xFF, 0x55, 0xFF, 0x00),
            Color::new(0xFF, 0xFF, 0x55, 0x00),
            Color::new(0xFF, 0xFF, 0xFF, 0x00),
            Color::new(0x00, 0x00, 0x00, 0x00),
            Color::new(0x10, 0x10, 0x10, 0x00),
            Color::new(0x20, 0x20, 0x20, 0x00),
            Color::new(0x35, 0x35, 0x35, 0x00),
            Color::new(0x45, 0x45, 0x45, 0x00),
            Color::new(0x55, 0x55, 0x55, 0x00),
            Color::new(0x65, 0x65, 0x65, 0x00),
            Color::new(0x75, 0x75, 0x75, 0x00),
            Color::new(0x8A, 0x8A, 0x8A, 0x00),
            Color::new(0x9A, 0x9A, 0x9A, 0x00),
            Color::new(0xAA, 0xAA, 0xAA, 0x00),
            Color::new(0xBA, 0xBA, 0xBA, 0x00),
            Color::new(0xCA, 0xCA, 0xCA, 0x00),
            Color::new(0xDF, 0xDF, 0xDF, 0x00),
            Color::new(0xEF, 0xEF, 0xEF, 0x00),
            Color::new(0xFF, 0xFF, 0xFF, 0x00),
            Color::new(0x00, 0x00, 0xFF, 0x00),
            Color::new(0x41, 0x00, 0xFF, 0x00),
            Color::new(0x82, 0x00, 0xFF, 0x00),
            Color::new(0xBE, 0x00, 0xFF, 0x00),
            Color::new(0xFF, 0x00, 0xFF, 0x00),
            Color::new(0xFF, 0x00, 0xBE, 0x00),
            Color::new(0xFF, 0x00, 0x82, 0x00),
            Color::new(0xFF, 0x00, 0x41, 0x00),
            Color::new(0xFF, 0x00, 0x00, 0x00),
            Color::new(0xFF, 0x41, 0x00, 0x00),
            Color::new(0xFF, 0x82, 0x00, 0x00),
            Color::new(0xFF, 0xBE, 0x00, 0x00),
            Color::new(0xFF, 0xFF, 0x00, 0x00),
            Color::new(0xBE, 0xFF, 0x00, 0x00),
            Color::new(0x82, 0xFF, 0x00, 0x00),
            Color::new(0x41, 0xFF, 0x00, 0x00),
            Color::new(0x00, 0xFF, 0x00, 0x00),
            Color::new(0x00, 0xFF, 0x41, 0x00),
            Color::new(0x00, 0xFF, 0x82, 0x00),
            Color::new(0x00, 0xFF, 0xBE, 0x00),
            Color::new(0x00, 0xFF, 0xFF, 0x00),
            Color::new(0x00, 0xBE, 0xFF, 0x00),
            Color::new(0x00, 0x82, 0xFF, 0x00),
            Color::new(0x00, 0x41, 0xFF, 0x00),
            Color::new(0x82, 0x82, 0xFF, 0x00),
            Color::new(0x9E, 0x82, 0xFF, 0x00),
            Color::new(0xBE, 0x82, 0xFF, 0x00),
            Color::new(0xDF, 0x82, 0xFF, 0x00),
            Color::new(0xFF, 0x82, 0xFF, 0x00),
            Color::new(0xFF, 0x82, 0xDF, 0x00),
            Color::new(0xFF, 0x82, 0xBE, 0x00),
            Color::new(0xFF, 0x82, 0x9E, 0x00),
            Color::new(0xFF, 0x82, 0x82, 0x00),
            Color::new(0xFF, 0x9E, 0x82, 0x00),
            Color::new(0xFF, 0xBE, 0x82, 0x00),
            Color::new(0xFF, 0xDF, 0x82, 0x00),
            Color::new(0xFF, 0xFF, 0x82, 0x00),
            Color::new(0xDF, 0xFF, 0x82, 0x00),
            Color::new(0xBE, 0xFF, 0x82, 0x00),
            Color::new(0x9E, 0xFF, 0x82, 0x00),
            Color::new(0x82, 0xFF, 0x82, 0x00),
            Color::new(0x82, 0xFF, 0x9E, 0x00),
            Color::new(0x82, 0xFF, 0xBE, 0x00),
            Color::new(0x82, 0xFF, 0xDF, 0x00),
            Color::new(0x82, 0xFF, 0xFF, 0x00),
            Color::new(0x82, 0xDF, 0xFF, 0x00),
            Color::new(0x82, 0xBE, 0xFF, 0x00),
            Color::new(0x82, 0x9E, 0xFF, 0x00),
            Color::new(0xBA, 0xBA, 0xFF, 0x00),
            Color::new(0xCA, 0xBA, 0xFF, 0x00),
            Color::new(0xDF, 0xBA, 0xFF, 0x00),
            Color::new(0xEF, 0xBA, 0xFF, 0x00),
            Color::new(0xFF, 0xBA, 0xFF, 0x00),
            Color::new(0xFF, 0xBA, 0xEF, 0x00),
            Color::new(0xFF, 0xBA, 0xDF, 0x00),
            Color::new(0xFF, 0xBA, 0xCA, 0x00),
            Color::new(0xFF, 0xBA, 0xBA, 0x00),
            Color::new(0xFF, 0xCA, 0xBA, 0x00),
            Color::new(0xFF, 0xDF, 0xBA, 0x00),
            Color::new(0xFF, 0xEF, 0xBA, 0x00),
            Color::new(0xFF, 0xFF, 0xBA, 0x00),
            Color::new(0xEF, 0xFF, 0xBA, 0x00),
            Color::new(0xDF, 0xFF, 0xBA, 0x00),
            Color::new(0xCA, 0xFF, 0xBA, 0x00),
            Color::new(0xBA, 0xFF, 0xBA, 0x00),
            Color::new(0xBA, 0xFF, 0xCA, 0x00),
            Color::new(0xBA, 0xFF, 0xDF, 0x00),
            Color::new(0xBA, 0xFF, 0xEF, 0x00),
            Color::new(0xBA, 0xFF, 0xFF, 0x00),
            Color::new(0xBA, 0xEF, 0xFF, 0x00),
            Color::new(0xBA, 0xDF, 0xFF, 0x00),
            Color::new(0xBA, 0xCA, 0xFF, 0x00),
            Color::new(0x00, 0x00, 0x71, 0x00),
            Color::new(0x1C, 0x00, 0x71, 0x00),
            Color::new(0x39, 0x00, 0x71, 0x00),
            Color::new(0x55, 0x00, 0x71, 0x00),
            Color::new(0x71, 0x00, 0x71, 0x00),
            Color::new(0x71, 0x00, 0x55, 0x00),
            Color::new(0x71, 0x00, 0x39, 0x00),
            Color::new(0x71, 0x00, 0x1C, 0x00),
            Color::new(0x71, 0x00, 0x00, 0x00),
            Color::new(0x71, 0x1C, 0x00, 0x00),
            Color::new(0x71, 0x39, 0x00, 0x00),
            Color::new(0x71, 0x55, 0x00, 0x00),
            Color::new(0x71, 0x71, 0x00, 0x00),
            Color::new(0x55, 0x71, 0x00, 0x00),
            Color::new(0x39, 0x71, 0x00, 0x00),
            Color::new(0x1C, 0x71, 0x00, 0x00),
            Color::new(0x00, 0x71, 0x00, 0x00),
            Color::new(0x00, 0x71, 0x1C, 0x00),
            Color::new(0x00, 0x71, 0x39, 0x00),
            Color::new(0x00, 0x71, 0x55, 0x00),
            Color::new(0x00, 0x71, 0x71, 0x00),
            Color::new(0x00, 0x55, 0x71, 0x00),
            Color::new(0x00, 0x39, 0x71, 0x00),
            Color::new(0x00, 0x1C, 0x71, 0x00),
            Color::new(0x39, 0x39, 0x71, 0x00),
            Color::new(0x45, 0x39, 0x71, 0x00),
            Color::new(0x55, 0x39, 0x71, 0x00),
            Color::new(0x61, 0x39, 0x71, 0x00),
            Color::new(0x71, 0x39, 0x71, 0x00),
            Color::new(0x71, 0x39, 0x61, 0x00),
            Color::new(0x71, 0x39, 0x55, 0x00),
            Color::new(0x71, 0x39, 0x45, 0x00),
            Color::new(0x71, 0x39, 0x39, 0x00),
            Color::new(0x71, 0x45, 0x39, 0x00),
            Color::new(0x71, 0x55, 0x39, 0x00),
            Color::new(0x71, 0x61, 0x39, 0x00),
            Color::new(0x71, 0x71, 0x39, 0x00),
            Color::new(0x61, 0x71, 0x39, 0x00),
            Color::new(0x55, 0x71, 0x39, 0x00),
            Color::new(0x45, 0x71, 0x39, 0x00),
            Color::new(0x39, 0x71, 0x39, 0x00),
            Color::new(0x39, 0x71, 0x45, 0x00),
            Color::new(0x39, 0x71, 0x55, 0x00),
            Color::new(0x39, 0x71, 0x61, 0x00),
            Color::new(0x39, 0x71, 0x71, 0x00),
            Color::new(0x39, 0x61, 0x71, 0x00),
            Color::new(0x39, 0x55, 0x71, 0x00),
            Color::new(0x39, 0x45, 0x71, 0x00),
            Color::new(0x51, 0x51, 0x71, 0x00),
            Color::new(0x59, 0x51, 0x71, 0x00),
            Color::new(0x61, 0x51, 0x71, 0x00),
            Color::new(0x69, 0x51, 0x71, 0x00),
            Color::new(0x71, 0x51, 0x71, 0x00),
            Color::new(0x71, 0x51, 0x69, 0x00),
            Color::new(0x71, 0x51, 0x61, 0x00),
            Color::new(0x71, 0x51, 0x59, 0x00),
            Color::new(0x71, 0x51, 0x51, 0x00),
            Color::new(0x71, 0x59, 0x51, 0x00),
            Color::new(0x71, 0x61, 0x51, 0x00),
            Color::new(0x71, 0x69, 0x51, 0x00),
            Color::new(0x71, 0x71, 0x51, 0x00),
            Color::new(0x69, 0x71, 0x51, 0x00),
            Color::new(0x61, 0x71, 0x51, 0x00),
            Color::new(0x59, 0x71, 0x51, 0x00),
            Color::new(0x51, 0x71, 0x51, 0x00),
            Color::new(0x51, 0x71, 0x59, 0x00),
            Color::new(0x51, 0x71, 0x61, 0x00),
            Color::new(0x51, 0x71, 0x69, 0x00),
            Color::new(0x51, 0x71, 0x71, 0x00),
            Color::new(0x51, 0x69, 0x71, 0x00),
            Color::new(0x51, 0x61, 0x71, 0x00),
            Color::new(0x51, 0x59, 0x71, 0x00),
            Color::new(0x00, 0x00, 0x41, 0x00),
            Color::new(0x10, 0x00, 0x41, 0x00),
            Color::new(0x20, 0x00, 0x41, 0x00),
            Color::new(0x31, 0x00, 0x41, 0x00),
            Color::new(0x41, 0x00, 0x41, 0x00),
            Color::new(0x41, 0x00, 0x31, 0x00),
            Color::new(0x41, 0x00, 0x20, 0x00),
            Color::new(0x41, 0x00, 0x10, 0x00),
            Color::new(0x41, 0x00, 0x00, 0x00),
            Color::new(0x41, 0x10, 0x00, 0x00),
            Color::new(0x41, 0x20, 0x00, 0x00),
            Color::new(0x41, 0x31, 0x00, 0x00),
            Color::new(0x41, 0x41, 0x00, 0x00),
            Color::new(0x31, 0x41, 0x00, 0x00),
            Color::new(0x20, 0x41, 0x00, 0x00),
            Color::new(0x10, 0x41, 0x00, 0x00),
            Color::new(0x00, 0x41, 0x00, 0x00),
            Color::new(0x00, 0x41, 0x10, 0x00),
            Color::new(0x00, 0x41, 0x20, 0x00),
            Color::new(0x00, 0x41, 0x31, 0x00),
            Color::new(0x00, 0x41, 0x41, 0x00),
            Color::new(0x00, 0x31, 0x41, 0x00),
            Color::new(0x00, 0x20, 0x41, 0x00),
            Color::new(0x00, 0x10, 0x41, 0x00),
            Color::new(0x20, 0x20, 0x41, 0x00),
            Color::new(0x28, 0x20, 0x41, 0x00),
            Color::new(0x31, 0x20, 0x41, 0x00),
            Color::new(0x39, 0x20, 0x41, 0x00),
            Color::new(0x41, 0x20, 0x41, 0x00),
            Color::new(0x41, 0x20, 0x39, 0x00),
            Color::new(0x41, 0x20, 0x31, 0x00),
            Color::new(0x41, 0x20, 0x28, 0x00),
            Color::new(0x41, 0x20, 0x20, 0x00),
            Color::new(0x41, 0x28, 0x20, 0x00),
            Color::new(0x41, 0x31, 0x20, 0x00),
            Color::new(0x41, 0x39, 0x20, 0x00),
            Color::new(0x41, 0x41, 0x20, 0x00),
            Color::new(0x39, 0x41, 0x20, 0x00),
            Color::new(0x31, 0x41, 0x20, 0x00),
            Color::new(0x28, 0x41, 0x20, 0x00),
            Color::new(0x20, 0x41, 0x20, 0x00),
            Color::new(0x20, 0x41, 0x28, 0x00),
            Color::new(0x20, 0x41, 0x31, 0x00),
            Color::new(0x20, 0x41, 0x39, 0x00),
            Color::new(0x20, 0x41, 0x41, 0x00),
            Color::new(0x20, 0x39, 0x41, 0x00),
            Color::new(0x20, 0x31, 0x41, 0x00),
            Color::new(0x20, 0x28, 0x41, 0x00),
            Color::new(0x2D, 0x2D, 0x41, 0x00),
            Color::new(0x31, 0x2D, 0x41, 0x00),
            Color::new(0x35, 0x2D, 0x41, 0x00),
            Color::new(0x3D, 0x2D, 0x41, 0x00),
            Color::new(0x41, 0x2D, 0x41, 0x00),
            Color::new(0x41, 0x2D, 0x3D, 0x00),
            Color::new(0x41, 0x2D, 0x35, 0x00),
            Color::new(0x41, 0x2D, 0x31, 0x00),
            Color::new(0x41, 0x2D, 0x2D, 0x00),
            Color::new(0x41, 0x31, 0x2D, 0x00),
            Color::new(0x41, 0x35, 0x2D, 0x00),
            Color::new(0x41, 0x3D, 0x2D, 0x00),
            Color::new(0x41, 0x41, 0x2D, 0x00),
            Color::new(0x3D, 0x41, 0x2D, 0x00),
            Color::new(0x35, 0x41, 0x2D, 0x00),
            Color::new(0x31, 0x41, 0x2D, 0x00),
            Color::new(0x2D, 0x41, 0x2D, 0x00),
            Color::new(0x2D, 0x41, 0x31, 0x00),
            Color::new(0x2D, 0x41, 0x35, 0x00),
            Color::new(0x2D, 0x41, 0x3D, 0x00),
            Color::new(0x2D, 0x41, 0x41, 0x00),
            Color::new(0x2D, 0x3D, 0x41, 0x00),
            Color::new(0x2D, 0x35, 0x41, 0x00),
            Color::new(0x2D, 0x31, 0x41, 0x00),
            Color::new(0x00, 0x00, 0x00, 0x00),
            Color::new(0x00, 0x00, 0x00, 0x00),
            Color::new(0x00, 0x00, 0x00, 0x00),
            Color::new(0x00, 0x00, 0x00, 0x00),
            Color::new(0x00, 0x00, 0x00, 0x00),
            Color::new(0x00, 0x00, 0x00, 0x00),
            Color::new(0x00, 0x00, 0x00, 0x00),
            Color::new(0x00, 0x00, 0x00, 0x00)
        ]);
    }
}
