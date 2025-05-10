use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, ImageData, Window};

#[wasm_bindgen]
pub fn draw_canvas_buffer() {
    // Obtenir la fenêtre et le document
    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");

    // Créer un élément canvas
    let canvas: HtmlCanvasElement = document
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()
        .unwrap();

    // Définir la taille du canvas
    let width: u32 = 256;
    let height: u32 = 256;
    canvas.set_width(width);
    canvas.set_height(height);

    // Ajouter le canvas au body du document
    document.body().unwrap().append_child(&canvas).unwrap();

    // Obtenir le contexte de rendu 2D
    let context: CanvasRenderingContext2d = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // Créer un buffer mémoire pour les pixels
    let buffer_size = (width as usize) * (height as usize) * 4; // 4 bytes par pixel (RGBA)
    let mut buffer = vec![0; buffer_size];

    // Remplir le buffer avec un fond bleu
    for y in 0..height {
        for x in 0..width {
            let index = ((y as usize) * (width as usize) + (x as usize)) * 4;
            buffer[index] = 0;     // R
            buffer[index + 1] = 0; // G
            buffer[index + 2] = 255; // B
            buffer[index + 3] = 255; // A
        }
    }

    // Dessiner un carré noir dans le buffer
    let square_size = 156;
    let square_x = 50;
    let square_y = 50;
    for y in square_y..square_y + square_size {
        for x in square_x..square_x + square_size {
            let index = ((y as usize) * (width as usize) + (x as usize)) * 4;
            buffer[index] = 0;     // R
            buffer[index + 1] = 0; // G
            buffer[index + 2] = 0; // B
            buffer[index + 3] = 255; // A
        }
    }

    // Créer un ImageData à partir du buffer
    let image_data = ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(&buffer), width, height).unwrap();

    // Dessiner l'ImageData sur le canvas
    context.put_image_data(&image_data, 0.0, 0.0).unwrap();
}
