//! Lineas con el algoritmo de Bresenham: solo sumas y comparaciones enteras.

use crate::framebuffer::Framebuffer;
use crate::matematica::Vec3;

/// Une dos puntos de pantalla (se usan `x` y `y`; `z` se ignora).
pub fn linea(fb: &mut Framebuffer, inicio: Vec3, fin: Vec3) {
    let (mut x0, mut y0) = (inicio.x.round() as i32, inicio.y.round() as i32);
    let (x1, y1) = (fin.x.round() as i32, fin.y.round() as i32);

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut error = dx + dy;

    loop {
        fb.punto(x0, y0);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * error;
        if e2 >= dy {
            error += dy;
            x0 += sx;
        }
        if e2 <= dx {
            error += dx;
            y0 += sy;
        }
    }
}
