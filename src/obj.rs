//! Lector de `.obj` a mano: solo los vertices (`v`) y las caras (`f`), que
//! es lo que hace falta para dibujar la malla.
//!
//! Las caras pueden venir como `v`, `v/vt`, `v//vn` o `v/vt/vn`; de cada
//! una solo se toma el indice del vertice. Una cara con mas de tres
//! vertices se parte en triangulos en abanico.

use crate::matematica::Vec3;
use std::fs;

/// Un triangulo: tres indices (desde 0) dentro de la lista de vertices.
pub struct Cara {
    pub indices: [usize; 3],
}

pub struct Modelo {
    pub vertices: Vec<Vec3>,
    pub caras: Vec<Cara>,
}

pub fn cargar_obj(ruta: &str) -> Result<Modelo, String> {
    let texto = fs::read_to_string(ruta).map_err(|e| format!("no se pudo leer {ruta}: {e}"))?;
    let mut vertices = Vec::new();
    let mut caras = Vec::new();

    for (n, linea) in texto.lines().enumerate() {
        let mut partes = linea.split_whitespace();
        match partes.next() {
            Some("v") => {
                let c: Vec<f32> = partes.take(3).map(|p| p.parse().unwrap_or(0.0)).collect();
                if c.len() < 3 {
                    return Err(format!("{ruta}:{}: vertice incompleto", n + 1));
                }
                vertices.push(Vec3::new(c[0], c[1], c[2]));
            }
            Some("f") => {
                let indices = partes
                    .map(|p| indice_vertice(p, vertices.len()))
                    .collect::<Option<Vec<usize>>>()
                    .ok_or_else(|| format!("{ruta}:{}: cara con un indice invalido", n + 1))?;
                for i in 1..indices.len().saturating_sub(1) {
                    caras.push(Cara { indices: [indices[0], indices[i], indices[i + 1]] });
                }
            }
            _ => {}
        }
    }
    Ok(Modelo { vertices, caras })
}

/// `"7//3"` -> 6. Los indices del `.obj` empiezan en 1, y los negativos
/// cuentan desde el ultimo vertice leido.
fn indice_vertice(token: &str, leidos: usize) -> Option<usize> {
    let i: i64 = token.split('/').next()?.parse().ok()?;
    let indice = if i > 0 { i - 1 } else { leidos as i64 + i };
    (0..leidos as i64).contains(&indice).then_some(indice as usize)
}
