//! El framebuffer: la imagen se pinta pixel por pixel en memoria y raylib
//! solo la sube a una textura para mostrarla.
//!
//! El color se guarda en RGBA de 8 bits, el mismo orden que la textura de
//! raylib, para subirlo sin convertir nada.

pub struct Framebuffer {
    pub ancho: usize,
    pub alto: usize,
    color: Vec<u8>,
    color_actual: [u8; 3],
}

impl Framebuffer {
    pub fn new(ancho: usize, alto: usize) -> Self {
        Framebuffer { ancho, alto, color: vec![255; ancho * alto * 4], color_actual: [255, 255, 255] }
    }

    /// Llena toda la imagen con el color de fondo.
    pub fn limpiar(&mut self, fondo: [u8; 3]) {
        for px in self.color.chunks_exact_mut(4) {
            px.copy_from_slice(&[fondo[0], fondo[1], fondo[2], 255]);
        }
    }

    /// El color con el que se pintan los siguientes puntos.
    pub fn set_color(&mut self, color: [u8; 3]) {
        self.color_actual = color;
    }

    /// Pinta un pixel con el color actual; lo que cae fuera de la pantalla se ignora.
    pub fn punto(&mut self, x: i32, y: i32) {
        if x < 0 || y < 0 || x as usize >= self.ancho || y as usize >= self.alto {
            return;
        }
        let i = (y as usize * self.ancho + x as usize) * 4;
        let [r, g, b] = self.color_actual;
        self.color[i..i + 4].copy_from_slice(&[r, g, b, 255]);
    }

    /// Los bytes RGBA, listos para `Texture2D::update_texture`.
    pub fn bytes(&self) -> &[u8] {
        &self.color
    }
}
