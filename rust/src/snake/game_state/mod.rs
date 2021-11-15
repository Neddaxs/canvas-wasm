const GAME_WIDTH: i32 = 32;
const GAME_SIZE: usize = (GAME_WIDTH * GAME_WIDTH) as usize;

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

        // let num = rand::thread_rng().gen_range(0..GAME_SIZE);

        let snake_position = 4;
        let apple_position = 1000;

        board[snake_position] = Tile::SNAKE;
        board[apple_position] = Tile::APPLE;

        State {
            board,
            snake: vec![snake_position as i32],
            apples: vec![apple_position as i32],
            direction: Direction::UP,
            apples_collected: 0,
            previous_best: previous_best.unwrap_or(0),
            running_state: RunningState::IDLE,
        }
    }

    fn move_snake(&mut self, eats_apple: bool) {
        // save current snake head position
        // shift all snake positions ie:
        // [1, 2, 8] === [?, 1, 2, 8]
        //
        // if eats_apple, keep the last item
        // else pop the last item off
        //
        // then take the ? and move it, according to the direction we want
    }

    fn board(&self) -> &[Tile; GAME_SIZE] {
        &self.board
    }
}
