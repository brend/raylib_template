use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;
const APP_TITLE: &str = "raylib template";

struct World {}

impl World {
    fn new() -> Self {
        World {}
    }
    
    fn update(&mut self) {}

    fn draw(&self, d: &mut RaylibDrawHandle) {}
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(APP_TITLE)
        .build();

    let mut world = World::new();

    while !rl.window_should_close() {
        world.update();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        world.draw(&mut d);
    }
}