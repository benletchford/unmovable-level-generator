use std::collections::BTreeMap;

// const PIECES: [Piece; 4] = [
//     Piece {
//         begin: Point {x: 0, y: 0},
//         end: Point {x: 1, y: 0},
//         horizontal: true
//     },
//     Piece {
//         begin: Point {x: 0, y: 0},
//         end: Point {x: 2, y: 0},
//         horizontal: true
//     },
//     Piece {
//         begin: Point {x: 0, y: 0},
//         end: Point {x: 0, y: 1},
//         horizontal: true
//     },
//     Piece {
//         begin: Point {x: 0, y: 0},
//         end: Point {x: 0, y: 2},
//         horizontal: true
//     }
// ];

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Point {
  pub x: u8,
  pub y: u8
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
#[derive(RustcDecodable, RustcEncodable)]
pub struct Piece {
    pub begin: Point,
    pub end: Point,
    pub horizontal: bool
}

impl Piece {
  pub fn new(begin: Point, end: Point) -> Piece {
    let horizontal = begin.y == end.y;

    Piece {
      begin: begin,
      end: end,
      horizontal: horizontal
    }
  }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Board {
  pub matrix: Vec<Vec<i8>>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Game {
  pub board: Board,
  pub width: usize,
  pub height: usize,
  pub pieces: Vec<Piece>,
  pub disabled_pieces: Vec<usize>,
  pub reversed_pieces: Vec<usize>,
  pub exit: bool,
  pub exit_point: Point,
  pub parent: Option<usize>
}

impl Game {
    pub fn new(width: usize, height: usize, prisoner: Piece, exit: bool) -> Game {
        let exit_point: Point;
        if prisoner.horizontal {
            if exit {
                exit_point = Point {x: width as u8 - 1, y: prisoner.begin.y};
            } else {
                exit_point = Point {x: 0, y: prisoner.begin.y};
            }
        } else {
            if exit {
                exit_point = Point {x: prisoner.begin.x, y: height as u8 - 1};
            } else {
                exit_point = Point {x: prisoner.begin.x, y: 0};
            }
        }

        let mut new_game = Game {
            width: width,
            height: height,
            board: Board {matrix: vec![vec![0; width]; height]},
            pieces: Vec::new(),
            disabled_pieces: Vec::new(),
            reversed_pieces: Vec::new(),
            exit: exit,
            exit_point: exit_point,
            parent: None
        };

        new_game.add(prisoner, false, false);
        new_game
    }

    pub fn string_to_game(game_str: &str, exit: bool) -> Game {
        let game_str = game_str.replace(" ", "");
        let game_rows: Vec<&str> = game_str.split("],").collect();

        let mut matrix: Vec<Vec<i8>> = Vec::new();
        for row_str in game_rows {
            let row_str = row_str
                .replace("[", "")
                .replace("]", "")
                .replace("\t", "")
                .replace("\n", "");

            let mut row: Vec<i8> = Vec::new();

            let numbers: Vec<&str> = row_str.split(",").collect();
            for number in numbers {
                row.push(number.parse::<i8>().unwrap());
            }
            matrix.push(row);
        }

        return Game::array_to_game(matrix, exit, Vec::new(), Vec::new());
    }

    pub fn array_to_game(array: Vec<Vec<i8>>, exit: bool, disabled_pieces: Vec<usize>, reversed_pieces: Vec<usize>) -> Game {
        let width = array.len();
        let height = array[0].len();

        let mut hashmap_of_points: BTreeMap<&i8, Vec<Point>> = BTreeMap::new();

        for y in 0..height {
            for x in 0..width {
                if array[y][x] == 0 { continue; }

                if !hashmap_of_points.contains_key(&array[y][x]) {
                    hashmap_of_points.insert(&array[y][x], Vec::new());
                }
                hashmap_of_points.get_mut(&array[y][x]).unwrap().push(
                    Point {x: x as u8, y: y as u8}
                )
            }
        }

        let mut pieces: Vec<Piece> = Vec::new();

        for (_, array_of_points) in hashmap_of_points.iter_mut() {
            let begin: Point = array_of_points.remove(0);

            let end: Point;
            if !array_of_points.is_empty() {
                let len = array_of_points.len();
                end = array_of_points.remove(len - 1);
            } else {
                end = begin.clone();
            }

            pieces.push(Piece::new(
                begin,
                end
            ));
        }

        let mut game = Game::new(width, height, pieces.remove(0), exit);

        let mut piece_index = 1;
        while !pieces.is_empty() {
            game.add(
                pieces.remove(0),
                disabled_pieces.iter().position(|&r| r == piece_index + 1).is_some(),
                reversed_pieces.iter().position(|&r| r == piece_index + 1).is_some()
            );

            piece_index += 1;
        }

        game
    }

    pub fn print(&self) {
        for row in &self.board.matrix {
            println!("{:?}", row);
        }
    }

    pub fn add(&mut self, piece: Piece, disabled: bool, reversed: bool) {
        self.pieces.push(piece);

        let len = self.pieces.len();

        // Get reference to the newly added piece
        let piece = &self.pieces[len - 1];

        if disabled {
            self.disabled_pieces.push(len);
        }
        if reversed {
            self.reversed_pieces.push(len);
        }
        // println!("{:?}", self.disabled_pieces);

        for x in piece.begin.x..(piece.end.x + 1) {
            for y in piece.begin.y..(piece.end.y + 1) {
                self.board.matrix[y as usize][x as usize] = len as i8;
            }
        }
    }

    pub fn check_can_add(&mut self, piece: Piece) -> bool {
        for x in piece.begin.x..(piece.end.x + 1) {
            for y in piece.begin.y..(piece.end.y + 1) {
                if self.board.matrix[y as usize][x as usize] != 0 {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn can_exit(&self) -> bool {
        let mut cells_that_need_to_be_empty: Vec<Point> = Vec::new();

        if self.pieces[0].horizontal {
            if self.exit {
                for i in (self.pieces[0].end.x + 1)..self.width as u8 {
                    cells_that_need_to_be_empty.push(Point {x: i, y: self.pieces[0].begin.y});
                }
            } else {
                for i in 0..self.pieces[0].begin.x {
                    cells_that_need_to_be_empty.push(Point {x: i, y: self.pieces[0].begin.y});
                }
            }
        } else {
        if self.exit {
            for i in (self.pieces[0].end.y + 1)..self.height as u8 {
                    cells_that_need_to_be_empty.push(Point {x: self.pieces[0].begin.x, y: i});
                }
        } else {
            for i in 0..self.pieces[0].begin.y {
                    cells_that_need_to_be_empty.push(Point {x: self.pieces[0].begin.x, y: i});
                }
            }
        }

        // Check that the cells are empty
        for cell in cells_that_need_to_be_empty {
            if self.board.matrix[cell.y as usize][cell.x as usize] != 0 {
                return false;
            }
        }

        true
    }

    pub fn move_piece(&mut self, piece_index: i8, direction: bool, steps: u8) -> bool {
        if self.disabled_pieces.iter().position(|&r| r == (piece_index + 1) as usize).is_some() {
            return false;
        }

        let piece = &mut self.pieces[piece_index as usize];

        let mut horizontal_movement = piece.horizontal;
        if self.reversed_pieces.iter().position(|&r| r == (piece_index + 1) as usize).is_some() {
            horizontal_movement = !horizontal_movement;
        }

        let mut cells_that_need_to_be_empty: Vec<Point> = Vec::new();
        if horizontal_movement {
            if direction {
                if (piece.end.x + steps) >= (self.width as u8) { return false }

                for x in (piece.end.x + 1)..piece.end.x + steps + 1 {
                    for y in piece.begin.y..piece.end.y + 1 {
                        cells_that_need_to_be_empty.push(Point {x: x, y: y});
                    }
                }
            } else {
                if (piece.begin.x as i8 - steps as i8) < 0 { return false }


                for x in (piece.begin.x - steps)..piece.begin.x {
                    for y in piece.begin.y..piece.end.y + 1 {
                        cells_that_need_to_be_empty.push(Point {x: x, y: y});
                    }
                }
            }
        } else {
            if direction {
                if (piece.end.y + steps) >= (self.height as u8) { return false }

                for x in piece.begin.x..piece.end.x + 1 {
                    for y in (piece.end.y + 1)..piece.end.y + steps + 1 {
                        cells_that_need_to_be_empty.push(Point {x: x, y: y});
                    }
                }
            }
            else {
                if (piece.begin.y as i8 - steps as i8) < 0 { return false; }

                for x in piece.begin.x..piece.end.x + 1 {
                    for y in (piece.begin.y - steps)..piece.begin.y {
                        cells_that_need_to_be_empty.push(Point {x: x, y: y});
                    }
                }
            }
        }

        // Check that the cells are empty
        for i in 0..cells_that_need_to_be_empty.len() {
            let cell = &cells_that_need_to_be_empty[i];

            if self.board.matrix[cell.y as usize][cell.x as usize] != piece_index + 1 &&
                self.board.matrix[cell.y as usize][cell.x as usize] != 0 {
                return false
            }
        }

        // Zero out old cells
        for x in piece.begin.x..piece.end.x + 1 {
            for y in piece.begin.y..piece.end.y + 1 {
                self.board.matrix[y as usize][x as usize] = 0;
            }
        }

        let mut new_begin = Point {x: piece.begin.x, y: piece.begin.y};
        let mut new_end   = Point {x: piece.end.x, y: piece.end.y};

        if horizontal_movement {
            if direction {
                new_begin.x += steps;
                new_end.x += steps;
            } else {
                new_begin.x -= steps;
                new_end.x -= steps;
            }
        } else {
            if direction {
                new_begin.y += steps;
                new_end.y += steps;
            } else {
                new_begin.y -= steps;
                new_end.y -= steps;
            }
        }

        piece.begin = new_begin;
        piece.end = new_end;

        for x in piece.begin.x..piece.end.x + 1 {
            for y in piece.begin.y..piece.end.y + 1 {
                self.board.matrix[y as usize][x as usize] = piece_index + 1;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, Piece, Point};

    #[test]
    fn array_to_game() {
        let prisoner = Piece::new(
            Point {x: 2, y: 2},
            Point {x: 3, y: 2}
        );
        let exit = true;
        let mut game = Game::new(6, 6, prisoner, exit);
        game.add(Piece::new(Point {x: 0, y: 0}, Point {x: 0, y: 2}), false, false);
        game.add(Piece::new(Point {x: 1, y: 1}, Point {x: 2, y: 1}), false, false);
        game.add(Piece::new(Point {x: 3, y: 0}, Point {x: 3, y: 1}), false, false);
        game.add(Piece::new(Point {x: 5, y: 0}, Point {x: 5, y: 1}), false, false);
        game.add(Piece::new(Point {x: 0, y: 3}, Point {x: 2, y: 3}), false, false);
        game.add(Piece::new(Point {x: 0, y: 5}, Point {x: 1, y: 5}), false, false);
        game.add(Piece::new(Point {x: 2, y: 4}, Point {x: 2, y: 5}), false, false);
        game.add(Piece::new(Point {x: 3, y: 5}, Point {x: 4, y: 5}), false, false);
        game.add(Piece::new(Point {x: 4, y: 2}, Point {x: 4, y: 4}), false, false);
        game.add(Piece::new(Point {x: 5, y: 3}, Point {x: 5, y: 4}), false, false);
        game.add(Piece::new(Point {x: 4, y: 0}, Point {x: 4, y: 0}), false, false);

        let converted_game = Game::array_to_game(vec![
            vec![2, 0, 0, 4, 12, 5],
            vec![2, 3, 3, 4, 0, 5],
            vec![2, 0, 1, 1, 10, 0],
            vec![6, 6, 6, 0, 10, 11],
            vec![0, 0, 8, 0, 10, 11],
            vec![7, 7, 8, 9, 9, 0]
        ], true, Vec::new(), Vec::new());

        game.print();
        converted_game.print();

        assert_eq!(game.board.matrix, converted_game.board.matrix);
    }

    #[test]
    fn string_to_game() {
        let converted_game = Game::string_to_game(
           "[[2, 0, 0, 4, 12, 5],
            [2, 3, 3, 4, 0, 5],
            [2, 0, 1, 1, 10, 0],
            [6, 6, 6, 0, 10, 11],
            [0, 0, 8, 0, 10, 11],
            [7, 7, 8, 9, 9, 0]]", true);

        assert_eq!(converted_game.board.matrix, [
            [2, 0, 0, 4, 12, 5],
            [2, 3, 3, 4, 0, 5],
            [2, 0, 1, 1, 10, 0],
            [6, 6, 6, 0, 10, 11],
            [0, 0, 8, 0, 10, 11],
            [7, 7, 8, 9, 9, 0]]
        );
    }
}

#[cfg(test)]
mod movement_tests {
    use super::Game;

    #[test]
    fn can_move_right() {
        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 4), true);
        assert_eq!(game.board.matrix, vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ]);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 1), true);
        assert_eq!(game.board.matrix, vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ]);
    }

    #[test]
    fn can_not_move_right() {
        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 2, 0],
            vec![0, 0, 0, 0, 2, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 2), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 2), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 1), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 1, 0, 2],
            vec![0, 0, 0, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 2), false);
    }

    #[test]
    fn can_move_left() {
        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 4), true);
        assert_eq!(game.board.matrix, vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ]);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 1), true);
        assert_eq!(game.board.matrix, vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ]);
    }

    #[test]
    fn can_not_move_left() {
        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 2, 1, 1, 0, 0],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 2), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 2), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 1), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![2, 0, 1, 1, 0, 0],
            vec![2, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 2), false);
    }

    #[test]
    fn can_move_down() {
        let mut game = Game::array_to_game(vec![
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 4), true);
        assert_eq!(game.board.matrix, vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0]
        ]);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 1), true);
        assert_eq!(game.board.matrix, vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0]
        ]);
    }

    #[test]
    fn can_not_move_down() {
        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 2, 2, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 2), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 2), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 1), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 2, 2, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, true, 2), false);
    }

    #[test]
    fn can_move_up() {
        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 4), true);
        assert_eq!(game.board.matrix, vec![
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ]);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 1), true);
        assert_eq!(game.board.matrix, vec![
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ]);
    }

    #[test]
    fn can_not_move_up() {
        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 2, 2, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 2), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 2), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 1), false);

        let mut game = Game::array_to_game(vec![
            vec![0, 0, 2, 2, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.move_piece(0, false, 2), false);
    }
}

#[cfg(test)]
mod exit_tests {
    use super::{Game};

    #[test]
    fn can_exit_right() {
        let game = Game::array_to_game(vec![
            vec![0, 3, 0, 0, 0, 0],
            vec![0, 3, 0, 0, 0, 0],
            vec![4, 1, 1, 0, 0, 0],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());

        assert_eq!(game.can_exit(), true);

        let game = Game::array_to_game(vec![
            vec![0, 3, 0, 0, 0, 0],
            vec![0, 3, 0, 0, 0, 0],
            vec![4, 0, 0, 0, 1, 1],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), true);
    }

    #[test]
    fn can_exit_left() {
        let game = Game::array_to_game(vec![
            vec![0, 3, 0, 0, 0, 0],
            vec![0, 3, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 4],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], false, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), true);

        let game = Game::array_to_game(vec![
            vec![0, 3, 0, 0, 0, 0],
            vec![0, 3, 0, 0, 0, 0],
            vec![1, 1, 0, 0, 4, 4],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], false, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), true);
    }

    #[test]
    fn can_exit_down() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 2, 2, 0, 0],
            vec![0, 4, 1, 3, 0, 0],
            vec![0, 4, 1, 3, 0, 0],
            vec![6, 6, 0, 0, 0, 0],
            vec![0, 0, 0, 5, 5, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), true);

        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 2, 2, 0, 0],
            vec![0, 4, 0, 3, 0, 0],
            vec![0, 4, 0, 3, 0, 0],
            vec![6, 6, 1, 0, 0, 0],
            vec![0, 0, 1, 5, 5, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), true);
    }

    #[test]
    fn can_exit_up() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 4, 1, 3, 0, 0],
            vec![0, 4, 1, 3, 0, 0],
            vec![6, 6, 2, 2, 0, 0],
            vec![0, 0, 0, 5, 5, 0]
        ], false, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), true);

        let game = Game::array_to_game(vec![
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 4, 0, 3, 0, 0],
            vec![0, 4, 0, 3, 0, 0],
            vec![6, 6, 2, 2, 0, 0],
            vec![0, 0, 0, 5, 5, 0]
        ], false, Vec::new(), Vec::new());

        game.print();

        assert_eq!(game.can_exit(), true);
    }

    #[test]
    fn can_not_exit_right() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 1, 1, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), false);

        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 1, 1, 2],
            vec![0, 0, 0, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), false);
    }

    #[test]
    fn can_not_exit_left() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![2, 1, 1, 0, 0, 0],
            vec![2, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], false, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), false);

        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 2, 0, 1, 1, 0],
            vec![0, 2, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], false, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), false);
    }

    #[test]
    fn can_not_exit_down() {
        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 2, 2, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), false);

        let game = Game::array_to_game(vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 2, 2, 0, 0, 0]
        ], true, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), false);
    }

    #[test]
    fn can_not_exit_up() {
        let game = Game::array_to_game(vec![
            vec![0, 2, 2, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], false, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), false);

        let game = Game::array_to_game(vec![
            vec![0, 0, 2, 2, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0, 0]
        ], false, Vec::new(), Vec::new());
        assert_eq!(game.can_exit(), false);
    }
}
