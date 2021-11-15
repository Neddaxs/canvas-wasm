use core::fmt;
use core::result::Result;
use rand::thread_rng;
use rand::Rng;

const GAME_WIDTH: i32 = 32;
const GAME_SIZE: usize = (GAME_WIDTH * GAME_WIDTH) as usize;

#[derive(Debug)]
pub enum SnakeDiedError {
    OffScreen,
}

impl std::error::Error for SnakeDiedError {}

impl fmt::Display for SnakeDiedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SnakeDiedError::OffScreen => write!(f, "Snake ran off screen"),
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

pub struct State {
    board: [Tile; GAME_SIZE],
    snake: Vec<i32>,
    apples: Vec<i32>,
    direction: Direction,
    apples_collected: i32,
    previous_best: i32,
    running_state: RunningState,
}

impl State {
    pub fn new(previous_best: Option<i32>) -> State {
        let mut board = [Tile::EMPTY; GAME_SIZE];

        let snake_position = thread_rng().gen_range(0..GAME_SIZE);

        board[snake_position] = Tile::SNAKE;

        let mut state = State {
            board,
            snake: vec![snake_position as i32],
            apples: vec![],
            direction: Direction::UP,
            apples_collected: 0,
            previous_best: previous_best.unwrap_or(0),
            running_state: RunningState::IDLE,
        };

        state.spawn_new_apple();
        state.board[state.apples[0] as usize] = Tile::APPLE;

        state
    }

    fn move_snake(&mut self) -> Result<(), SnakeDiedError> {
        let new_snake_head_position = match self.direction {
            Direction::UP => self.snake[0] - GAME_WIDTH,
            Direction::RIGHT => self.snake[0] + 1,
            Direction::DOWN => self.snake[0] - 1,
            Direction::LEFT => self.snake[0] + GAME_WIDTH,
        };

        let ok = if new_snake_head_position < 0 {
            false
        } else if (new_snake_head_position as usize) > GAME_SIZE {
            false
        } else {
            true
        };

        // TODO, handle the left and right bounds
        if !ok {
            return Err(SnakeDiedError::OffScreen);
        }

        let new_snake_head_position_as_usize = new_snake_head_position as usize;

        self.snake.insert(new_snake_head_position_as_usize, 0);

        let mut eats_apple = false;
        for apple_position in self.apples.iter() {
            if *apple_position == new_snake_head_position {
                eats_apple = true;
                break;
            }
        }

        self.board[new_snake_head_position_as_usize] = Tile::SNAKE;

        if eats_apple {
            self.apples_collected += 1;
            self.spawn_new_apple();
            self.apples.remove(new_snake_head_position_as_usize);
        } else {
            let last = self.snake.pop().unwrap();
            self.board[last as usize] = Tile::EMPTY;
        }

        Ok(())
    }

    fn spawn_new_apple(&mut self) {
        let index = self.new_non_colliding_index();
        self.apples.push(index);
        self.board[index as usize] = Tile::APPLE;
    }

    fn new_non_colliding_index(&self) -> i32 {
        3
    }

    fn board(&self) -> &[Tile; GAME_SIZE] {
        &self.board
    }
}
