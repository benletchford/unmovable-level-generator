extern crate rustc_serialize;
use rustc_serialize::json;

extern crate rand;
use rand::{thread_rng, sample};

mod game;
use game::Game;
mod solver;
// use solver::solve;

mod generate;
use generate::generate;

use std::env;

use game::{Piece};


#[derive(RustcDecodable, RustcEncodable)]
pub struct GameData {
  pub game: Vec<Vec<i8>>,
  pub steps: Vec<Vec<Vec<i8>>>,
  pub difficulty: i8,
  pub disabled_pieces: Vec<usize>,
  pub reversed_pieces: Vec<usize>,
  pub prisoner: Piece,
  pub width: i8,
  pub height: i8,
}

fn main() {
    // let game_str = env::args().nth(1).unwrap();
    // let game = Game::string_to_game(&game_str, true);
    let minimum_difficulty = env::args().nth(1).unwrap().parse::<i8>().unwrap();
    let disabled_bias = env::args().nth(2).unwrap().parse::<i8>().unwrap();
    let reverse_bias = env::args().nth(3).unwrap().parse::<i8>().unwrap();

    let games = vec![
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new()),
        Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0, 0]
            ], true, Vec::new(), Vec::new())
    ];

    let mut rng = thread_rng();
    let game = games[sample(&mut rng, 0..games.len(), 1)[0] as usize].clone();
    // println!("{:?}", sample(&mut rng, 0..games.len(), 1)[0]);
    loop {
        let games_option = generate(game.clone(), disabled_bias, reverse_bias);
        // println!(".");
        match games_option {
            Some(games) => {
                // println!("Found a game of difficulty: {:?}", games.len() as i8);
                if (games.len() as i8) < minimum_difficulty {
                    continue;
                }
                // if games[0].disabled_pieces.len() == 0 {
                //     continue;
                // }

                let mut game_data = GameData {
                    game: games[0].board.matrix.clone(),
                    steps: Vec::new(),
                    difficulty: (games.len() as i8) - 1,
                    prisoner: game.pieces[0].clone(),
                    width: game.width as i8,
                    height: game.height as i8,
                    disabled_pieces: games[0].disabled_pieces.clone(),
                    reversed_pieces: games[0].reversed_pieces.clone()
                };

                for i in 1..games.len() {
                    game_data.steps.push(games[i].board.matrix.clone());
                }

                println!("{}", json::encode(&game_data).unwrap());
                break;
            },
            None => {
                continue;
            }
        }
    }
}
