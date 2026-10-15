//! Lo minimo de algebra que hace falta: un vector de tres componentes y
//! las rotaciones para girar el modelo antes de ponerlo en pantalla.

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    /// Gira alrededor del eje Y (angulo en radianes).
    pub fn rotar_y(self, angulo: f32) -> Self {
        let (s, c) = angulo.sin_cos();
        Vec3::new(c * self.x + s * self.z, self.y, -s * self.x + c * self.z)
    }

    /// Gira alrededor del eje X (angulo en radianes).
    pub fn rotar_x(self, angulo: f32) -> Self {
        let (s, c) = angulo.sin_cos();
        Vec3::new(self.x, c * self.y - s * self.z, s * self.y + c * self.z)
    }
}
