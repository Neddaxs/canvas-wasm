use core::fmt;
use core::result::Result;
use rand::thread_rng;
use rand::Rng;

use super::utils::keys::KeyValue;

const GAME_WIDTH: i32 = 32;
const GAME_SIZE: usize = (GAME_WIDTH * GAME_WIDTH) as usize;

#[derive(Debug)]
pub enum SnakeDiedError {
    OffScreen,
    HitSelf,
}

impl std::error::Error for SnakeDiedError {}

impl fmt::Display for SnakeDiedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnakeDiedError::OffScreen => write!(f, "Snake ran off screen!"),
            SnakeDiedError::HitSelf => write!(f, "Snake ran into itself!"),
        }
    }
}

pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

pub enum RunningState {
    IDLE,
    RUNNING,
    PAUSED,
}

#[derive(Copy, Clone)]
pub enum Tile {
    SNAKE,
    EMPTY,
    APPLE,
}

#[derive(Copy, Clone)]
pub struct GridTile {
    top: Option<usize>,
    right: Option<usize>,
    bottom: Option<usize>,
    left: Option<usize>,
    pub row: i32,
    pub col: i32,
    pub index: usize,
    pub state: Tile,
}

impl GridTile {
    fn empty() -> GridTile {
        GridTile {
            top: None,
            right: None,
            bottom: None,
            left: None,
            col: 0,
            row: 0,
            index: 0,
            state: Tile::EMPTY,
        }
    }
}

pub struct State {
    board: [GridTile; GAME_SIZE],
    snake: Vec<usize>,
    apples: Vec<usize>,
    direction: Direction,
    apples_collected: i32,
    previous_best: i32,
    running_state: RunningState,
}

impl State {
    pub fn new(previous_best: Option<i32>) -> State {
        let mut board = init_board();

        let snake_position = thread_rng().gen_range((GAME_WIDTH as usize)..GAME_SIZE);

        board[snake_position].state = Tile::SNAKE;

        let mut state = State {
            board,
            snake: vec![snake_position],
            apples: vec![],
            direction: Direction::UP,
            apples_collected: 0,
            previous_best: previous_best.unwrap_or(0),
            running_state: RunningState::IDLE,
        };

        state.spawn_new_apple();
        state.board[state.apples[0]].state = Tile::APPLE;

        state
    }

    pub fn move_snake(&mut self) -> Result<(), SnakeDiedError> {
        let snake_head = &self.board[self.snake[0]];
        let new_snake_index_option = match self.direction {
            Direction::UP => snake_head.top,
            Direction::RIGHT => snake_head.right,
            Direction::DOWN => snake_head.bottom,
            Direction::LEFT => snake_head.left,
        };

        match new_snake_index_option {
            Some(new_snake_index) => {
                let mut new_snake_head_tile = self.board[new_snake_index];

                match new_snake_head_tile.state {
                    Tile::APPLE => {
                        self.apples_collected += 1;
                        self.spawn_new_apple();
                        new_snake_head_tile.state = Tile::SNAKE;
                    }
                    Tile::EMPTY => {
                        new_snake_head_tile.state = Tile::SNAKE;
                        self.snake.insert(0, new_snake_index);

                        let last = self.snake.pop().unwrap();
                        self.board[last].state = Tile::EMPTY;
                    }
                    Tile::SNAKE => {
                        return Err(SnakeDiedError::HitSelf);
                    }
                }
            }
            None => return Err(SnakeDiedError::OffScreen),
        }

        Ok(())
    }

    fn spawn_new_apple(&mut self) {
        let index = self.new_non_colliding_index();
        self.apples.push(index);
        self.board[index as usize].state = Tile::APPLE;
    }

    fn new_non_colliding_index(&self) -> usize {
        let mut position: usize;

        loop {
            position = thread_rng().gen_range(32..GAME_SIZE);

            match self.board.get(position) {
                Some(tile) => match tile.state {
                    Tile::EMPTY => {
                        position = tile.index;
                        break;
                    }
                    _ => {}
                },
                None => {}
            }
        }

        position
    }

    pub fn board(&self) -> &[GridTile; GAME_SIZE] {
        &self.board
    }

    pub fn snake(&self) -> &Vec<usize> {
        &self.snake
    }

    pub fn change_direction(&mut self, direction: KeyValue) {
        match direction {
            KeyValue::DownArrow => self.direction = Direction::DOWN,
            KeyValue::UpArrow => self.direction = Direction::UP,
            KeyValue::RightArrow => self.direction = Direction::RIGHT,
            KeyValue::LeftArrow => self.direction = Direction::LEFT,
            _ => {}
        }
    }
}

fn init_board() -> [GridTile; GAME_SIZE] {
    let mut board = [GridTile::empty(); GAME_SIZE];

    let mut row = -1;
    let mut col = -1;

    for i in 0..GAME_SIZE {
        if i % (GAME_WIDTH as usize) == 0 {
            row += 1;
            col = -1;
        }

        col += 1;

        board[i].index = i;
        board[i].col = col;
        board[i].row = row;
    }

    for tile in board {
        let index = tile.index as i32;

        board[tile.index].left = safe_get(index - 1, &board).and_then(|tile| Some(tile.index));
        board[tile.index].right = safe_get(index + 1, &board).and_then(|tile| Some(tile.index));
        board[tile.index].top =
            safe_get(index - GAME_WIDTH, &board).and_then(|tile| Some(tile.index));
        board[tile.index].bottom =
            safe_get(index + GAME_WIDTH, &board).and_then(|tile| Some(tile.index));
    }

    board
}

fn safe_get<T>(index: i32, slice: &[T]) -> Option<&T> {
    if index >= 0 {
        return slice.get(index as usize);
    }

    None
}

pub fn tile_size() -> i32 {
    return (GAME_SIZE as i32) / GAME_WIDTH;
}
