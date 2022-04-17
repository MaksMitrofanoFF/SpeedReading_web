extern crate core;

use cgmath::{Deg, Matrix4, SquareMatrix, vec3};
use std::fs::File;
// use std::intrinsics::{sinf32, sinf64};
use std::io::Read;
use notan::app::Event;
use notan::draw::*;
use notan::prelude::*;
use crate::keyboard::KeyCode;

#[derive(AppState)]
struct State {
    font: Font,
    dragging: usize,
    asset: Option<Asset<String>>,
    words: Vec<String>,
    speed: f32,
    current_word: f32,
    background_color: Color,
    text_color: Color,
    text_size: f32
}

// Create a new asset loaded to load .txt files as strings
fn create_text_loader() -> AssetLoader {
    AssetLoader::new().use_parser(parse_text).extension("txt")
}

// This parses the &[u8] from the file to the type that we want, string in this case
fn parse_text(_id: &str, data: Vec<u8>) -> Result<String, String> {
    String::from_utf8(data).map_err(|e| e.to_string())
}

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_loader(create_text_loader())
        .add_config(DrawConfig)
        .add_config(WindowConfig::new().title("SpeedReading"))
        .add_config(notan::log::LogConfig::new(notan::log::LevelFilter::Debug))
        .draw(draw)
        .event(event)
        .update(update)
        .build()
}

fn setup(gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/ubuntu.ttf"))
        .unwrap();
    State {
        font,
        dragging: 0,
        words: Vec::new(),
        speed: 0.0,
        current_word: 0.0,
        asset: None,
        background_color: Color::BLACK,
        text_color: Color::WHITE,
        text_size: 32.0,
    }
}

fn event(assets: &mut Assets, state: &mut State, evt: Event) {
    match evt {
        Event::DragEnter { .. } => {
            state.dragging += 1;
        }
        Event::DragLeft => {
            state.dragging = 0;
        }
        Event::MouseWheel { delta_x, delta_y } => {
            state.speed -= 10.0 * delta_y.signum();
            if state.speed <= -180.0 {
                state.speed = -180.0;
            }
        }

        Event::Drop(file) => {
            state.dragging = 0;

            if file.mime == "text/plain" {
                state.asset = Some(assets.load_dropped_file::<String>(&file).unwrap());
            }
        }
        _ => {}
    }
}

fn update(app: &mut App, state: &mut State) {

    if app.keyboard.is_down(KeyCode::Q) {
        state.background_color = Color::RED;
    }

    if app.keyboard.is_down(KeyCode::W) {
        state.background_color = Color::GREEN;
    }
    if app.keyboard.is_down(KeyCode::E) {
        state.background_color = Color::BLUE;
    }

    if app.keyboard.is_down(KeyCode::R) {
        state.background_color = Color::AQUA;
    }

    if app.keyboard.is_down(KeyCode::T) {
        state.background_color = Color::MAROON;
    }

    if app.keyboard.is_down(KeyCode::Y) {
        state.background_color = Color::ORANGE;
    }

    if app.keyboard.is_down(KeyCode::U) {
        state.background_color = Color::NAVY;
    }

    if app.keyboard.is_down(KeyCode::I) {
        state.background_color = Color::WHITE;
    }

    if app.keyboard.is_down(KeyCode::O) {
        state.background_color = Color::BLACK;
    }

    if app.keyboard.is_down(KeyCode::P) {
        state.text_color = Color::BLACK;
    }

    if app.keyboard.is_down(KeyCode::A) {
        state.text_color = Color::WHITE;
    }

    if app.keyboard.is_down(KeyCode::V) && app.keyboard.is_down(KeyCode::C) {
        state.text_color = Color::BLACK;
        state.background_color = Color::WHITE;
    }

    if app.keyboard.is_down(KeyCode::Z) && app.keyboard.is_down(KeyCode::X) {
        state.text_color = Color::WHITE;
        state.background_color = Color::BLACK;
    }

    if app.mouse.left_is_down() {
        state.speed = 0.0;
    }

    if app.keyboard.is_down(KeyCode::F) {
        state.background_color = Color::MAGENTA;
    }

    if app.keyboard.is_down(KeyCode::G) {
        state.text_color = Color::RED;
    }

    if app.keyboard.is_down(KeyCode::H) {
        state.text_color = Color::GREEN;
    }
    if app.keyboard.is_down(KeyCode::J) {
        state.text_color = Color::BLUE;
    }

    if app.keyboard.is_down(KeyCode::K) {
        state.text_color = Color::AQUA;
    }

    if app.keyboard.is_down(KeyCode::L) {
        state.text_color = Color::MAROON;
    }

    if app.keyboard.is_down(KeyCode::B) {
        state.text_color = Color::ORANGE;
    }

    if app.keyboard.is_down(KeyCode::N) {
        state.text_color = Color::NAVY;
    }

    if app.mouse.left_is_down() {
        state.speed = 0.0;
    }

    if app.keyboard.is_down(KeyCode::M) {
        state.text_color = Color::MAGENTA;
    }

    let time = app.timer.time_since_init();
    let r = time.sin() * 0.5 + 0.5;
    let g = (time + 5.0).sin() * 0.5 + 0.5;
    let b = (time + 10.0).sin() * 0.5 + 0.5;

    if app.keyboard.is_down(KeyCode::Key1) && app.keyboard.is_down(KeyCode::Key2) && app.keyboard.is_down(KeyCode::Key3){
        state.background_color = Color::new(r,g,b,1.0);
        state.text_color = Color::new(b,r,g,1.0);
    }

     if app.keyboard.is_down(KeyCode::Up) {
         state.text_size += 2.0
     }
    if app.keyboard.is_down(KeyCode::Down) {
        state.text_size -= 2.0
    }
}

fn draw(app: &mut App, gfx: &mut Graphics, state: &mut State) {
    let frame_time = app.timer.delta_f32();

    let mut draw = gfx.create_draw();
    draw.clear(state.background_color);

    if state.words.is_empty() {
        if let Some(asset) = &state.asset {
            if asset.is_loaded() {
                let text = state.asset.take().unwrap().try_unwrap().unwrap();
                for part in text.split([' ', '\n']) {
                    state.words.push(part.to_owned());
                }
            }
        }
    }

    // Just UI Text
    if state.dragging == 0 {
        if state.words.is_empty() {
            let text = "Перетащите сюда файл .txt";
            draw.text(&state.font, text)
                .color(Color::ORANGE)
                .size(30.0)
                .v_align_middle()
                .h_align_center()
                .position(400.0, 300.0);
        } else {
            let index = (state.current_word as usize).min(state.words.len() - 1);
            let word = &state.words[index];
            draw.text(&state.font, word)
                .color(state.text_color)
                .size(state.text_size)
                .v_align_middle()
                .h_align_center()
                .position(400.0, 300.0);

            draw.text(&state.font, &format!("{} слов в минуту", state.speed))
                .color(state.text_color)
                .size(24.0)
                .v_align_middle()
                .position(25.0, 25.0);

            state.current_word += state.speed / 60.0 * frame_time;
        }
    } else {
        draw.rect((10.0, 10.0), (780.0, 580.0))
            .color(Color::WHITE)
            .stroke(6.0);

        let text = format!("Вы перетаскиваете {} файл", state.dragging);
        draw.text(&state.font, &text)
            .size(30.0)
            .color(Color::GRAY)
            .v_align_middle()
            .h_align_center()
            .position(400.0, 300.0);
    }

    gfx.render(&draw);
}