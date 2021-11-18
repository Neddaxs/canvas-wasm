use rand::thread_rng;
use rand::Rng;

use super::utils::keys::KeyValue;

const GAME_WIDTH: i32 = 32;
const GAME_HEIGHT: i32 = 32;
const GAME_SIZE: usize = (GAME_WIDTH * GAME_HEIGHT) as usize;

const BASE_FPS: f64 = 5.0;

#[derive(Debug, PartialEq)]
pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(PartialEq)]
pub enum RunningState {
    IDLE,
    RUNNING,
    PAUSED,
    DIED,
}

#[derive(Copy, Clone, PartialEq)]
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
    pub apples_collected: i32,
    pub previous_best: i32,
    pub running_state: RunningState,
    pub fps: f64,
}

impl State {
    pub fn new(previous_best: Option<i32>) -> State {
        let board = init_board();

        let mut state = State {
            board,
            snake: vec![],
            apples: vec![],
            direction: Direction::UP,
            apples_collected: 0,
            previous_best: previous_best.unwrap_or(0),
            running_state: RunningState::IDLE,
            fps: BASE_FPS,
        };

        state.spawn_snake();
        state.spawn_new_apple();

        state
    }

    fn reset(&mut self) {
        let high_score = if self.previous_best > self.apples_collected {
            self.previous_best
        } else {
            self.apples_collected
        };
        self.apples_collected = 0;
        self.previous_best = high_score;
        self.board = init_board();
        self.apples = vec![];
        self.snake = vec![];
        self.direction = Direction::UP;
        self.fps = BASE_FPS;
        self.spawn_snake();
        self.spawn_new_apple();
    }

    pub fn move_snake(&mut self) {
        let snake_head = &self.board[self.snake[0]];
        let new_snake_index_option = match self.direction {
            Direction::UP => snake_head.top,
            Direction::RIGHT => snake_head.right,
            Direction::DOWN => snake_head.bottom,
            Direction::LEFT => snake_head.left,
        };

        match new_snake_index_option {
            Some(new_snake_index) => match self.board[new_snake_index].state {
                Tile::SNAKE => {
                    for snake_index in self.snake.iter().cloned() {
                        if self.board[snake_index].state == Tile::SNAKE {
                            self.running_state = RunningState::DIED;
                            break;
                        }
                    }
                }
                Tile::APPLE => {
                    self.apples_collected += 1;
                    self.spawn_new_apple();
                    self.snake.insert(0, new_snake_index);
                    self.board[new_snake_index].state = Tile::SNAKE;

                    if self.apples_collected % 3 == 0 {
                        self.fps += 0.5;
                    }
                }
                Tile::EMPTY => {
                    self.snake.insert(0, new_snake_index);

                    let last = self.snake.pop().unwrap();
                    self.board[last].state = Tile::EMPTY;

                    self.board[new_snake_index].state = Tile::SNAKE;
                }
            },
            None => {
                // Went off map
                self.running_state = RunningState::DIED;
            }
        }
    }

    pub fn toggle_game(&mut self) {
        match self.running_state {
            RunningState::IDLE | RunningState::PAUSED => self.running_state = RunningState::RUNNING,
            RunningState::RUNNING => self.running_state = RunningState::PAUSED,
            RunningState::DIED => {
                self.reset();
                self.running_state = RunningState::RUNNING;
            }
        }
    }

    fn spawn_snake(&mut self) {
        let index = self.new_non_colliding_index();
        self.snake.push(index);
        self.board[index as usize].state = Tile::SNAKE;
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

    pub fn change_direction(&mut self, direction: KeyValue) {
        match direction {
            KeyValue::DownArrow => {
                if self.direction != Direction::UP {
                    self.direction = Direction::DOWN;
                }
            }
            KeyValue::UpArrow => {
                if self.direction != Direction::DOWN {
                    self.direction = Direction::UP
                }
            }
            KeyValue::RightArrow => {
                if self.direction != Direction::LEFT {
                    self.direction = Direction::RIGHT;
                }
            }
            KeyValue::LeftArrow => {
                if self.direction != Direction::RIGHT {
                    self.direction = Direction::LEFT
                }
            }
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

        board[tile.index].left = safe_get(index - 1, &board).and_then(|next_tile| {
            if next_tile.col < tile.col {
                Some(next_tile.index)
            } else {
                None
            }
        });
        board[tile.index].right = safe_get(index + 1, &board).and_then(|next_tile| {
            if next_tile.col > tile.col {
                Some(next_tile.index)
            } else {
                None
            }
        });
        board[tile.index].top =
            safe_get(index - GAME_WIDTH, &board).and_then(|next_tile| Some(next_tile.index));
        board[tile.index].bottom =
            safe_get(index + GAME_WIDTH, &board).and_then(|next_tile| Some(next_tile.index));
    }

    board
}

fn safe_get<T>(index: i32, slice: &[T]) -> Option<&T> {
    if index >= 0 {
        return slice.get(index as usize);
    }

    None
}

pub fn tile_size(aspect: f64) -> f64 {
    (100.0 / GAME_WIDTH as f64) * aspect
}
