//! SR1: cargar un `.obj` y dibujarlo con la funcion de triangulos.
//!
//! Se recorren las caras a mano: cada una trae tres indices, con ellos se
//! sacan los tres vertices y se pinta el triangulo en el framebuffer.
//! raylib solo pone la ventana y la textura donde se muestra el framebuffer.
//!
//! `cargo run` abre la ventana (P guarda `render.png`).
//! `cargo run -- --captura` guarda `render.png` y se cierra.

mod framebuffer;
mod linea;
mod matematica;
mod obj;
mod triangulo;

use framebuffer::Framebuffer;
use matematica::Vec3;
use obj::{cargar_obj, Modelo};
use raylib::prelude::*;
use triangulo::triangulo;

const ANCHO: usize = 800;
const ALTO: usize = 600;
const MODELO: &str = "modelos/personaje_simi_piloto.obj";
const CAPTURA: &str = "render.png";
const FONDO: [u8; 3] = [18, 16, 32];

/// Que parte del alto de la pantalla ocupa el modelo.
const OCUPACION: f32 = 0.75;
/// El modelo mira hacia -Z: media vuelta para verlo de frente, mas un poco
/// de giro y de inclinacion para que se note el volumen.
const GIRO_Y: f32 = std::f32::consts::PI - 0.5;
const INCLINACION_X: f32 = 0.35;

/// Pasa cada vertice del modelo a coordenadas de pantalla: lo gira, lo
/// escala para que ocupe `OCUPACION` del alto y lo centra.
fn a_pantalla(vertices: &[Vec3]) -> Vec<Vec3> {
    let girados: Vec<Vec3> = vertices.iter().map(|v| v.rotar_y(GIRO_Y).rotar_x(INCLINACION_X)).collect();

    let (mut min, mut max) = (Vec3::new(f32::MAX, f32::MAX, f32::MAX), Vec3::new(f32::MIN, f32::MIN, f32::MIN));
    for v in &girados {
        min = Vec3::new(min.x.min(v.x), min.y.min(v.y), min.z.min(v.z));
        max = Vec3::new(max.x.max(v.x), max.y.max(v.y), max.z.max(v.z));
    }
    let centro = Vec3::new((min.x + max.x) / 2.0, (min.y + max.y) / 2.0, (min.z + max.z) / 2.0);
    let escala = (OCUPACION * ALTO as f32 / (max.y - min.y)).min(OCUPACION * ANCHO as f32 / (max.x - min.x));

    // En pantalla Y crece hacia abajo, por eso se le cambia el signo.
    girados
        .iter()
        .map(|v| Vec3::new(ANCHO as f32 / 2.0 + (v.x - centro.x) * escala, ALTO as f32 / 2.0 - (v.y - centro.y) * escala, (v.z - centro.z) * escala))
        .collect()
}

fn render(fb: &mut Framebuffer, modelo: &Modelo, vertices: &[Vec3]) {
    for cara in &modelo.caras {
        let [i0, i1, i2] = cara.indices;
        triangulo(fb, vertices[i0], vertices[i1], vertices[i2]);
    }
}

fn guardar(textura: &Texture2D) {
    let _ = std::fs::remove_file(CAPTURA);
    if let Ok(imagen) = textura.load_image() {
        imagen.export_image(CAPTURA);
    }
    if std::path::Path::new(CAPTURA).exists() {
        println!("captura guardada en {CAPTURA}");
    } else {
        eprintln!("no se pudo guardar {CAPTURA}");
    }
}

fn main() {
    let solo_captura = std::env::args().any(|a| a == "--captura");

    let modelo = cargar_obj(MODELO).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    println!("{MODELO}: {} vertices, {} caras", modelo.vertices.len(), modelo.caras.len());
    let vertices = a_pantalla(&modelo.vertices);

    let (mut rl, thread) = raylib::init().size(ANCHO as i32, ALTO as i32).title("Software Renderer").log_level(TraceLogLevel::LOG_WARNING).build();
    rl.set_target_fps(60);

    let mut textura = rl.load_texture_from_image(&thread, &Image::gen_image_color(ANCHO as i32, ALTO as i32, Color::BLACK)).expect("no se pudo crear la textura");
    let mut fb = Framebuffer::new(ANCHO, ALTO);

    while !rl.window_should_close() {
        fb.limpiar(FONDO);
        fb.set_color([255, 255, 0]);
        render(&mut fb, &modelo, &vertices);
        textura.update_texture(fb.bytes()).expect("no se pudo actualizar la textura");

        if solo_captura || rl.is_key_pressed(KeyboardKey::KEY_P) {
            guardar(&textura);
            if solo_captura {
                break;
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&textura, 0, 0, Color::WHITE);
    }
}
