use game::{Game, Piece, Point};
use solver::{solve, extra_solve};

extern crate rand;
use rand::{thread_rng, sample};


// #[derive(Debug)]
// #[derive(Clone)]
// #[derive(PartialEq)]
// pub struct Piece {
//     pub games: Vec<Games>,
//     pub horizontal: bool
// }
//

pub fn generate(mut initial_game: Game, disabled_bias: i8, reverse_bias: i8) -> Option<Vec<Game>> {
    let pieces: Vec<Piece> = vec![
        Piece::new(
            Point {x: 0, y: 0},
            Point {x: 1, y: 0}
        ),
        Piece::new(
            Point {x: 0, y: 0},
            Point {x: 2, y: 0}
        ),
        Piece::new(
            Point {x: 0, y: 0},
            Point {x: 0, y: 1}
        ),
        Piece::new(
            Point {x: 0, y: 0},
            Point {x: 0, y: 2}
        )
    ];
    let mut rng = thread_rng();

    for x in 0..initial_game.width {
        for y in 0..initial_game.height {
            let rand_i = sample(&mut rng, 0..pieces.len() + 2, 1)[0];
            if rand_i >= pieces.len() {
                continue;
            }

            let mut disabled = false;
            let rand_j = sample(&mut rng, 0..100, 1)[0];
            if rand_j <= disabled_bias {
                disabled = true;
            }

            let mut reverse = false;
            let rand_k = sample(&mut rng, 0..100, 1)[0];
            if rand_k <= reverse_bias && !disabled {
                reverse = true;
            }

            let mut piece = pieces[rand_i as usize].clone();

            // Shift dem u guiz
            piece.begin.x += x as u8;
            piece.end.x += x as u8;
            piece.begin.y += y as u8;
            piece.end.y += y as u8;

            // Make sure there's actually room u guysz
            if piece.end.x >= initial_game.width as u8 { continue };
            if piece.end.y >= initial_game.height as u8 { continue };

            if initial_game.check_can_add(piece.clone()) {
                initial_game.add(piece, disabled, reverse);
            }
        }
    }

    let solved_option = solve(&initial_game);
    match solved_option {
        Some(_) => {
            return Some(extra_solve(&initial_game).unwrap());
        },
        None => {
            return None;
        }
    };
}
