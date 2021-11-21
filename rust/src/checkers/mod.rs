struct Renderer {
    root: HTMLDivElement,
    canvas: HTMLCanvasElement,
}

enum Playing {
    Active,
    Paused,
    Dead,
}

enum Scene {
    Menu,
    Playing(Playing),
}

struct Game {
    tiles: vec![usize],
    scene: Scene,
    game_state: GameState,
}
