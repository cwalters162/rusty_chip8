use macroquad::input::KeyCode;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;
use macroquad::ui::root_ui;

use std::env;
use std::fs::File;
use std::io::Read;

use chip8::chip8::*;

#[macroquad::main("Chip-8 Emulator")]
async fn main() {
    const SCALE: u32 = 10;
    const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
    const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run path/to/game");
        return;
    }

    let mut chip8_system = Chip8::new();
    let mut rom = File::open(&args[1]).expect("Unable to open file.");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();
    chip8_system.load(&buffer);

    set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT);

    'gameloop: loop {
        if is_key_pressed(KeyCode::Escape) {
            break 'gameloop;
        }

        if is_key_down(KeyCode::Key1) {
            chip8_system.key_press(0x1, true);
        } else if is_key_down(KeyCode::Key2) {
            chip8_system.key_press(0x2, true);
        } else if is_key_down(KeyCode::Key3) {
            chip8_system.key_press(0x3, true);
        } else if is_key_down(KeyCode::Key4) {
            chip8_system.key_press(0xC, true);
        } else if is_key_down(KeyCode::Q) {
            chip8_system.key_press(0x4, true);
        } else if is_key_down(KeyCode::W) {
            chip8_system.key_press(0x5, true);
        } else if is_key_down(KeyCode::E) {
            chip8_system.key_press(0x6, true);
        } else if is_key_down(KeyCode::R) {
            chip8_system.key_press(0xD, true);
        } else if is_key_down(KeyCode::A) {
            chip8_system.key_press(0x7, true);
        } else if is_key_down(KeyCode::S) {
            chip8_system.key_press(0x8, true);
        } else if is_key_down(KeyCode::D) {
            chip8_system.key_press(0x9, true);
        } else if is_key_down(KeyCode::Z) {
            chip8_system.key_press(0xE, true);
        } else if is_key_down(KeyCode::X) {
            chip8_system.key_press(0x0, true);
        } else if is_key_down(KeyCode::C) {
            chip8_system.key_press(0xB, true);
        } else if is_key_down(KeyCode::V) {
            chip8_system.key_press(0xF, true);
        } else {
            for hex in 0..16usize {
                chip8_system.key_press(hex, false);
            }
        }

        for _ in 0..9 {
            chip8_system.tick();
        }
        chip8_system.tick_timers();

        let screen_buffer = chip8_system.get_display();

        for (i, pixel) in screen_buffer.iter().enumerate() {
            if *pixel {
                let x = (i % 64) as f32;
                let y = (i / 64) as f32;
                root_ui().canvas().rect(
                    Rect::new(
                        (x * SCALE as f32),
                        (y * SCALE as f32),
                        SCALE as f32,
                        SCALE as f32,
                    ),
                    WHITE,
                    WHITE,
                );
            }
        }

        next_frame().await
    }
}

// fn key_mapping(key: KeyCode) -> Option<usize> {
//     match key {
//         KeyCode::Key1 => Some(0x1),
//         KeyCode::Key2 => Some(0x2),
//         KeyCode::Key3 => Some(0x3),
//         KeyCode::Key4 => Some(0xC),
//
//         KeyCode::Q => Some(0x4),
//         KeyCode::W => Some(0x5),
//         KeyCode::E => Some(0x6),
//         KeyCode::R => Some(0xD),
//
//         KeyCode::A => Some(0x7),
//         KeyCode::S => Some(0x8),
//         KeyCode::D => Some(0x9),
//         KeyCode::F => Some(0xE),
//
//         KeyCode::Z => Some(0xA),
//         KeyCode::X => Some(0x0),
//         KeyCode::C => Some(0xB),
//         KeyCode::V => Some(0xF),
//         _ => None,
//     }
// }
