//! Un triangulo es el contorno que forman sus tres vertices.

use crate::framebuffer::Framebuffer;
use crate::linea::linea;
use crate::matematica::Vec3;

pub fn triangulo(fb: &mut Framebuffer, a: Vec3, b: Vec3, c: Vec3) {
    linea(fb, a, b);
    linea(fb, b, c);
    linea(fb, c, a);
}
