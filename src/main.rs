//! This program solves the [English peg solitaire puzzle](http://en.wikipedia.org/wiki/Peg_solitaire)
//! and can produce image and gif solutions. The solver and input puzzle format is based on a
//! [Golang example](https://go.dev/test/solitaire.go). The code extends the existing sample so that it can work
//! with other variations of the puzzle, allowing the user to define a custom board.

use std::{env, path::Path};

use rust_peg_solver::{
    solution::{create_gif, create_images, print_solution},
    Solver,
};

/// Output type of the solution
///
/// The solution can be printed in the console ([`PrintText`]). Alternatively an image for every step
/// ([`CreateImages`]) or a single GIF file ([`CreateGif`]) can be created.
pub enum SolutionMode {
    PrintText,
    CreateImages,
    CreateGif,
}

/// Main function of the program.
///
/// Example call:
///
/// cargo run -- games/english.txt images solutions
///
/// - The first argument is the path of the input file
/// - The second argument is the solution output mode (text, images, gif). Default is text.
/// - The third argument is the output folder for the images. Default is solutions.
pub fn main() {
    let mut args = env::args();
    args.next();

    let file_path = match args.next() {
        Some(arg) => arg,
        None => String::from("games/english.txt"),
    };

    let mode: SolutionMode = match args.next() {
        Some(arg) => match arg.as_str() {
            "text" => SolutionMode::PrintText,
            "images" => SolutionMode::CreateImages,
            "gif" => SolutionMode::CreateGif,
            _ => SolutionMode::PrintText,
        },
        None => SolutionMode::PrintText,
    };

    let output_folder = match args.next() {
        Some(arg) => arg,
        None => String::from("solutions"),
    };

    if !Path::new(&output_folder).is_dir() {
        panic!("Output folder doesn't exist");
    }

    let mut solver = Solver::init_from_file(&file_path).unwrap();
    solver.solve();
    match mode {
        SolutionMode::PrintText => print_solution(&solver.solution),
        SolutionMode::CreateImages => create_images(&solver.solution, &output_folder),
        SolutionMode::CreateGif => create_gif(&solver.solution, &output_folder),
    }
}
