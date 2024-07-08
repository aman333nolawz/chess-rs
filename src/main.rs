mod chess;
use std::collections::HashMap;

use chess::{get_nth_bit, Chess};
use macroquad::prelude::*;

const W: f32 = 800.0;
const H: f32 = 800.0;
const SQ: f32 = W / 8.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chess".to_owned(),
        fullscreen: false,
        window_width: W as i32,
        window_height: H as i32,
        ..Default::default()
    }
}

fn draw_board(
    chess: &Chess,
    piece_textures: &HashMap<char, Texture2D>,
    selected: Option<u8>,
    legal_moves: u64,
) {
    let colors = [Color::from_hex(0xf3f3f4), Color::from_hex(0x6a9b41)];
    let texture_params = DrawTextureParams {
        dest_size: Some(Vec2::new(SQ, SQ)),
        ..Default::default()
    };

    for i in 0..64 {
        let x = (i % 8) as f32;
        let y = (i / 8) as f32;
        draw_rectangle(x * SQ, y * SQ, SQ, SQ, colors[(x + y) as usize % 2]);

        let piece = chess.get_piece_at(63 - i);
        if piece.is_some() {
            draw_texture_ex(
                &piece_textures[&piece.unwrap().get_char()],
                x * SQ,
                y * SQ,
                WHITE,
                texture_params.clone(),
            );
        }
        if selected.is_some() && i == selected.unwrap() {
            draw_rectangle_lines(x * SQ, y * SQ, SQ, SQ, 10.0, BLUE);
        }

        if get_nth_bit(legal_moves, 63 - i) == 1 {
            draw_circle(
                x * SQ + SQ / 2.0,
                y * SQ + SQ / 2.0,
                SQ / 4.0,
                Color::from_rgba(27, 27, 27, 100),
            );
        }
    }
}

async fn load_textures() -> HashMap<char, Texture2D> {
    let mut textures = HashMap::new();
    textures.insert('P', load_texture("images/P.png").await.unwrap());
    textures.insert('N', load_texture("images/N.png").await.unwrap());
    textures.insert('B', load_texture("images/B.png").await.unwrap());
    textures.insert('R', load_texture("images/R.png").await.unwrap());
    textures.insert('Q', load_texture("images/Q.png").await.unwrap());
    textures.insert('K', load_texture("images/K.png").await.unwrap());

    textures.insert('p', load_texture("images/p.png").await.unwrap());
    textures.insert('n', load_texture("images/n.png").await.unwrap());
    textures.insert('b', load_texture("images/b.png").await.unwrap());
    textures.insert('r', load_texture("images/r.png").await.unwrap());
    textures.insert('q', load_texture("images/q.png").await.unwrap());
    textures.insert('k', load_texture("images/k.png").await.unwrap());
    return textures;
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut chess = Chess::new();

    let piece_textures = load_textures().await;
    let mut selected = None;
    let mut legal_moves = 0;

    loop {
        clear_background(BLACK);
        draw_board(&chess, &piece_textures, selected, legal_moves);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mut x, mut y) = mouse_position();
            x /= SQ;
            y /= SQ;
            let i = x as u8 + y as u8 * 8;

            if selected.is_none() {
                if 0.0 < x && x < 8.0 && 0.0 < y && y < 8.0 {
                    selected = Some(i);
                    legal_moves = chess.legal_moves(63 - selected.unwrap(), chess.turn);
                }
            } else {
                chess.move_piece(63 - selected.unwrap(), 63 - i as u8);
                selected = None;
                legal_moves = 0;
            }
        }

        next_frame().await
    }
}
