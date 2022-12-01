use std::cmp;
use std::collections::VecDeque;

use game::{Game, Board};

#[derive(Debug)]
#[derive(Clone)]
// #[derive(PartialEq)]
pub struct Node {
    pub game: Game,
    pub last: Option<Box<Node>>
}

pub fn solve(initial_game: &Game) -> Option<Game> {
    let mut checked: Vec<Board> = Vec::new();
    let mut queue: VecDeque<Game> = VecDeque::new();

    let max_steps = cmp::max(initial_game.width, initial_game.height);

    queue.push_back(initial_game.clone());

    while !queue.is_empty() {
        // We know for sure there is at least one thing in the queue
        let game = queue.pop_front().unwrap();

        if checked.len() > 1000 {
            return None;
        }
        if game.can_exit() {
            return Some(game.clone());
        }
        else if game_in(&game, &checked) == false {
            checked.push(game.board.clone());

            for i in 0..game.pieces.len() {
                for j in 1..max_steps {
                    let mut pos_game = game.clone();
                    if pos_game.move_piece(i as i8, true, j as u8) {
                        queue.push_back(pos_game);
                    } else {
                        break;
                    }
                }

                for j in 1..max_steps {
                    let mut neg_game = game.clone();
                    if neg_game.move_piece(i as i8, false, j as u8) {
                        queue.push_back(neg_game);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    return None;
}

pub fn extra_solve(initial_game: &Game) -> Option<Vec<Game>> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut checked: Vec<Board> = Vec::new();
    let mut queue: VecDeque<Node> = VecDeque::new();

    let max_steps = cmp::max(initial_game.width, initial_game.height);

    nodes.push(
        Node {
            game: initial_game.clone(),
            last: None
        }
    );
    queue.push_back(Node {
        game: initial_game.clone(),
        last: None
    });

    while !queue.is_empty() {
        // We know for sure there is at least one thing in the queue
        let game_node = queue.pop_front().unwrap();

        if game_node.game.can_exit() {
            let mut games = Vec::new();

            let mut solved_option_node = Some(Box::new(game_node.clone()));
            loop {
                match solved_option_node {
                    Some(node) => {
                        games.push((*node).game.clone());
                        solved_option_node = node.last;
                    },
                    None => {break;}
                };
            }

            games.reverse();

            return Some(games);
        }
        else if game_in(&game_node.game, &checked) == false {
            checked.push(game_node.game.board.clone());

            for i in 0..game_node.game.pieces.len() {
                for j in 1..max_steps {
                    let mut pos_game = game_node.game.clone();
                    if pos_game.move_piece(i as i8, true, j as u8) {
                        queue.push_back(Node {
                            game: pos_game.clone(),
                            last: Some(Box::new(game_node.clone()))
                        });
                        nodes.push(Node {
                            game: pos_game.clone(),
                            last: Some(Box::new(game_node.clone()))
                        });
                    } else {
                        break;
                    }
                }

                for j in 1..max_steps {
                    let mut neg_game = game_node.game.clone();
                    if neg_game.move_piece(i as i8, false, j as u8) {
                        queue.push_back(Node {
                            game: neg_game.clone(),
                            last: Some(Box::new(game_node.clone()))
                        });
                        nodes.push(Node {
                            game: neg_game.clone(),
                            last: Some(Box::new(game_node.clone()))
                        });
                    } else {
                        break;
                    }
                }
            }
        }
    }

    return None;
}

fn game_in(game: &Game, vec: &Vec<Board>) -> bool {
  for board in vec {
    if *game.board.matrix == *board.matrix {
      return true;
    }
  }

  false
}

#[cfg(test)]
mod tests {
    use game::Game;
    use super::solve;

    #[test]
    fn can_solve_easy_game() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 2, 0],
            vec![0, 0, 1, 1, 2, 3],
            vec![0, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 0, 3],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());

        let solved_game = solve(&game).unwrap();

        assert_eq!(solved_game.board.matrix, vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 2, 3],
            vec![0, 0, 0, 0, 2, 3]
        ]);
    }

    #[test]
    fn can_not_solve_game_with_reversed_piece() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ], true, Vec::new(), vec![2]);

        assert_eq!(solve(&game).is_none(), true);
    }

    #[test]
    fn can_solve_game_with_reversed_piece() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 2, 2],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ], true, Vec::new(), vec![2]);

        let solved_game = solve(&game).unwrap();

        assert_eq!(solved_game.board.matrix, vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 2, 2],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ]);
    }

    #[test]
    fn can_not_solve_game_with_disabled_piece() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ], true, vec![2], Vec::new());

        assert_eq!(solve(&game).is_none(), true);
    }

    #[test]
    fn can_solve_game_with_disabled_piece() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 4, 0],
            vec![0, 0, 0, 0, 4, 0],
            vec![1, 1, 0, 0, 0, 2],
            vec![0, 3, 3, 3, 0, 2],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ], true, vec![3, 4], Vec::new());

        let solved_game = solve(&game).unwrap();

        assert_eq!(solved_game.board.matrix, vec![
            vec![0, 0, 0, 0, 4, 0],
            vec![0, 0, 0, 0, 4, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 3, 3, 3, 0, 2],
            vec![0, 0, 0, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 0],
        ]);
    }

    #[test]
    fn can_medium_game() {
        let game = Game::array_to_game(vec![
            vec![2, 0, 0, 4, 0, 5],
            vec![2, 3, 3, 4, 0, 5],
            vec![2, 0, 1, 1, 10, 0],
            vec![6, 6, 6, 0, 10, 11],
            vec![0, 0, 8, 0, 10, 11],
            vec![7, 7, 8, 9, 9, 0]
        ], true, Vec::new(), Vec::new());

        let solved_game = solve(&game).unwrap();

        assert_eq!(solved_game.board.matrix, vec![
            vec![0, 0, 8, 4, 0, 5],
            vec![3, 3, 8, 4, 0, 5],
            vec![2, 1, 1, 0, 0, 0],
            vec![2, 6, 6, 6, 10, 0],
            vec![2, 0, 0, 0, 10, 11],
            vec![7, 7, 9, 9, 10, 11]
        ]);
    }

    #[test]
    fn can_solve_tricky_game() {
        let game = Game::array_to_game(vec![
            vec![2, 3, 3, 0, 5, 6],
            vec![2, 0, 4, 0, 5, 6],
            vec![0, 0, 4, 1, 1, 7],
            vec![11, 11, 10, 9, 0, 7],
            vec![0, 0, 10, 9, 8, 8],
            vec![12, 12, 12, 9, 0, 0]
        ], true, Vec::new(), Vec::new());

        let solved_game = solve(&game).unwrap();

        assert_eq!(solved_game.board.matrix, vec![
            vec![2, 0, 4, 3, 3, 6],
            vec![2, 0, 4, 0, 0, 6],
            vec![1, 1, 0, 0, 0, 0],
            vec![11, 11, 10, 9, 5, 7],
            vec![8, 8, 10, 9, 5, 7],
            vec![12, 12, 12, 9, 0, 0]
        ]);
    }
}
