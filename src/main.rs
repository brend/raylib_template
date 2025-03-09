use rand::Rng;
use raylib::prelude::*;

const GRAVITY: f32 = 0.000001;

struct Particle {
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    color: Color,
    size: f32,
    age: f32,
}

impl Particle {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..800.0);
        let y = 600.0;
        let vx = 0.0;
        let vy = rng.gen_range(-0.05..-0.01);
        let ax = 0.0;
        let ay = 0.0;
        let r = rng.gen_range(0..255);
        let g = rng.gen_range(0..255);
        let b = rng.gen_range(0..255);
        let a = 255;
        let size = rng.gen_range(1.0..5.0);

        Self {
            position: Vector2::new(x, y),
            velocity: Vector2::new(vx, vy),
            acceleration: Vector2::new(ax, ay),
            color: Color::new(r, g, b, a),
            size,
            age: 0.0,
        }
    }

    fn apply_force(&mut self, force: Vector2) {
        self.acceleration += force;
    }

    fn update(&mut self) {
        self.age += 1.0;
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        let c = Color::new(self.color.r, self.color.g, self.color.b, (255.0 - self.age / 100.0) as u8);
        d.draw_circle_v(self.position, self.size, c);
    }
}

struct World {
    particles: Vec<Particle>,
}

impl World {
    fn update(&mut self) {
        let g = Vector2::new(0.0, GRAVITY);
        self.particles.retain(|p| p.velocity.y < 0.0);
        if rand::thread_rng().gen_range(0.0..1.0) < 0.01 {
            self.particles.push(Particle::random());
        } 
        for particle in self.particles.iter_mut() {
            particle.apply_force(g);
            particle.update();
        }
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        for particle in self.particles.iter() {
            particle.draw(d);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Particle Playground")
        .build();

    let mut world = World {
        particles: Vec::new(),
    };

    let mut i = 0;

    while !rl.window_should_close() {
        world.update();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        world.draw(&mut d);

        i += 1;

        if i % 100 == 0 {
            println!("Particles: {}", world.particles.len());
        }
    }
}