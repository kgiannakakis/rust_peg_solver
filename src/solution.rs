//! Outputs solutions in various formats (text, images, gif files).

use gif::{Encoder, Repeat};
use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::error::Error;
use std::fs::File;
use std::time::Instant;

use crate::board::{clear_border, GameBoard, GameMove, MoveDirection};

/// Output tile width in pixels
const TILE_WIDTH: u32 = 60;

/// Output tile height in pixels
const TILE_HEIGHT: u32 = 60;

// Gif frame delay in units of 10ms
const FRAME_DELAY: u16 = 50;

/// Make GIF animations repeat infinitely
const REPEAT_INFINITE: bool = true;

/// Creates a board image
fn create_board_image(image_name: &str, game_move: &GameMove) -> Result<(), Box<dyn Error>> {
    let GameBoard {
        board,
        row_count,
        column_count,
    } = clear_border(&game_move.board);

    let mut image = ImageReader::open("images/wood.png")?.decode()?;
    image = image.resize(
        column_count * TILE_WIDTH,
        row_count * TILE_HEIGHT,
        image::imageops::FilterType::CatmullRom,
    );

    let mut glass = ImageReader::open("images/glass.png")?.decode()?;
    let mut hole = ImageReader::open("images/hole.png")?.decode()?;

    glass = glass.resize(
        TILE_WIDTH,
        TILE_WIDTH,
        image::imageops::FilterType::CatmullRom,
    );
    hole = hole.resize(
        TILE_WIDTH,
        TILE_WIDTH,
        image::imageops::FilterType::CatmullRom,
    );

    for x in 0..column_count {
        for y in 0..row_count {
            let ch = board[(y * (column_count + 1) + x) as usize];
            if ch == '●' {
                image::imageops::overlay(
                    &mut image,
                    &glass,
                    i64::from(x * TILE_WIDTH),
                    i64::from(y * TILE_HEIGHT),
                );
            } else if ch == '○' {
                image::imageops::overlay(
                    &mut image,
                    &hole,
                    i64::from(x * TILE_WIDTH),
                    i64::from(y * TILE_HEIGHT),
                );
            }
        }
    }

    image.save(image_name)?;

    Ok(())
}

/// Adds a frame to the GIF animation
fn add_frame(
    image: &DynamicImage,
    frame_width: u16,
    frame_height: u16,
    encoder: &mut Encoder<&mut File>,
    i: usize,
) -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let mut pixels: Vec<u8> = image.to_rgba8().into_raw();
    let mut frame = gif::Frame::from_rgba_speed(frame_width, frame_height, &mut pixels, 10);
    frame.delay = FRAME_DELAY;
    let duration = start.elapsed();
    println!("Created frame {}: {:?}", i + 1, duration);
    let start = Instant::now();
    encoder.write_frame(&frame)?;
    let duration = start.elapsed();
    println!("Added frame {}: {:?}", i + 1, duration);

    Ok(())
}

/// Creates a solution in GIF format
fn create_solution_gif(image_name: &str, solution: &[GameMove]) -> Result<(), Box<dyn Error>> {
    let GameBoard {
        row_count,
        column_count,
        ..
    } = clear_border(&solution[0].board);

    let frame_width = (column_count * TILE_WIDTH) as u16;
    let frame_height = (row_count * TILE_HEIGHT) as u16;

    let mut background = ImageReader::open("images/wood.png")?.decode()?;
    background = background.resize(
        column_count * TILE_WIDTH,
        row_count * TILE_HEIGHT,
        image::imageops::FilterType::CatmullRom,
    );

    let mut glass = ImageReader::open("images/glass.png")?.decode()?;
    let mut hole = ImageReader::open("images/hole.png")?.decode()?;
    let mut glass_selected = ImageReader::open("images/glass_selected.png")?.decode()?;
    let mut hole_selected = ImageReader::open("images/hole_selected.png")?.decode()?;

    glass = glass.resize(
        TILE_WIDTH,
        TILE_WIDTH,
        image::imageops::FilterType::CatmullRom,
    );
    hole = hole.resize(
        TILE_WIDTH,
        TILE_WIDTH,
        image::imageops::FilterType::CatmullRom,
    );
    glass_selected = glass_selected.resize(
        TILE_WIDTH,
        TILE_WIDTH,
        image::imageops::FilterType::CatmullRom,
    );
    hole_selected = hole_selected.resize(
        TILE_WIDTH,
        TILE_WIDTH,
        image::imageops::FilterType::CatmullRom,
    );

    let mut image = File::create(image_name)?;
    let mut encoder = gif::Encoder::new(&mut image, frame_width, frame_height, &[])?;
    if REPEAT_INFINITE {
        encoder.set_repeat(Repeat::Infinite)?;
    }

    for (i, game_move) in solution.iter().enumerate() {
        let board = crate::solution::clear_border(&game_move.board).board;

        let mut image = background.clone();

        for x in 0..column_count {
            for y in 0..row_count {
                let tile_pos = (y * (column_count + 1) + x) as usize;
                let ch = board[tile_pos];
                if ch == '●' {
                    image::imageops::overlay(
                        &mut image,
                        &glass,
                        i64::from(x * TILE_WIDTH),
                        i64::from(y * TILE_HEIGHT),
                    );
                } else if ch == '○' {
                    image::imageops::overlay(
                        &mut image,
                        &hole,
                        i64::from(x * TILE_WIDTH),
                        i64::from(y * TILE_HEIGHT),
                    );
                }
            }
        }
        add_frame(&image, frame_width, frame_height, &mut encoder, i)?;
        let pos = game_move.start_pos;
        let dir = &game_move.direction;

        if pos == 0 && *dir == MoveDirection::Still {
            break;
        }

        let start_pos_row: u32 = (pos as u32 / (column_count + 5)) - 2;
        let start_pos_column: u32 = (pos as u32 % (column_count + 5)) - 2;

        let target_pos_row: u32;
        let target_pos_column: u32;

        match dir {
            MoveDirection::Up => {
                target_pos_row = start_pos_row - 2;
                target_pos_column = start_pos_column;
            }
            MoveDirection::Down => {
                target_pos_row = start_pos_row + 2;
                target_pos_column = start_pos_column;
            }
            MoveDirection::Left => {
                target_pos_row = start_pos_row;
                target_pos_column = start_pos_column - 2;
            }
            MoveDirection::Right => {
                target_pos_row = start_pos_row;
                target_pos_column = start_pos_column + 2;
            }
            MoveDirection::Still => {
                target_pos_row = start_pos_row;
                target_pos_column = start_pos_column;
            }
        };

        image::imageops::overlay(
            &mut image,
            &glass_selected,
            i64::from(start_pos_column * TILE_WIDTH),
            i64::from(start_pos_row * TILE_HEIGHT),
        );

        image::imageops::overlay(
            &mut image,
            &hole_selected,
            i64::from(target_pos_column * TILE_WIDTH),
            i64::from(target_pos_row * TILE_HEIGHT),
        );

        add_frame(&image, frame_width, frame_height, &mut encoder, i)?;
    }

    Ok(())
}

/// Prints solution as text in the console
pub fn print_solution(solution: &[GameMove]) {
    for game_move in solution.iter() {
        let board = clear_border(&game_move.board).board;
        println!("{}", String::from_iter(board));
        println!("{} {:?}\n", game_move.start_pos, game_move.direction);
    }
}

/// Creates solution steps as images saved in the `output_folder`.
pub fn create_images(solution: &[GameMove], output_folder: &str) {
    for (i, game_move) in solution.iter().enumerate() {
        create_board_image(
            &format!("{}/solution_{:03}.png", output_folder, i + 1),
            game_move,
        )
        .unwrap();
    }
}

/// Creates a GIF animation of the solution. Saves a GIF file in `output_folder`.
pub fn create_gif(solution: &[GameMove], output_folder: &str) {
    create_solution_gif(&format!("{}/solution.gif", output_folder), solution).unwrap();
}
