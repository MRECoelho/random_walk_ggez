extern crate ggez;
extern crate rand;

use ggez::event::EventHandler;
use ggez::{GameResult, Context, event, graphics, timer};
use ggez::conf::Conf;
use ggez::graphics::{circle, DrawMode, present, Point2, Color, set_color};

use rand::random;
use std::fs::File;

const TARGET_FPS: u32 = 60;

struct GameState {
    walkers: Vec<RandomWalker>,
    width: f32,
    height: f32
}

impl GameState {
    fn new(width: f32, height: f32) -> GameResult<GameState> {
        let mut walkers = Vec::new();

        // for _ in 1..5 {
        //     walkers.push(RandomWalker::new(width, height)?);
        // }

        walkers.push(RandomWalker::new(width, height)?);
        // walkers.push(RandomWalker::new(width, height)?);
        // walkers.push(RandomWalker::new(width, height)?);

        Ok(GameState {
            walkers,
            width,
            height
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        while timer::check_update_time(context, TARGET_FPS) {
            let dt = 1.0 / TARGET_FPS as f32;

            for walker in &mut self.walkers {
                walker.update(self.width, self.height, dt);
                walker.keep_in_arena(self.width, self.height)?;
                walker.bullet.update(context, dt, self.width, self.height)?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::clear(context);

        for walker in &mut self.walkers {
            walker.draw(context)?;
            walker.bullet.draw(context)?;
        }

        present(context);
        Ok(())
    }
}

struct RandomWalker {
    location: Point2,
    radius: f32,
    color: Color,
    bullet: Bullet,
    destination: Point2,
    velocity: Point2,
    speed: f32
}

impl RandomWalker {
    fn new(width: f32, height: f32) -> GameResult<RandomWalker> {
        let x = width / 2.0;
        let y = height / 2.0;
        let color = Color::new( random::<f32>(), 
                                random::<f32>(), 
                                random::<f32>(), 
                                1.0);
        let bullet = Bullet::new();
        let destination = Point2::new(x, y);
        let velocity = Point2::new(0f32, 0f32);
        let speed = 100f32;

        Ok(RandomWalker {
            location: Point2::new(x, y),
            radius: 15.0,
            color,
            bullet,
            destination,
            velocity,
            speed
        })
    }

    fn update(&mut self, game_width: f32, game_height: f32, dt: f32) {
        if !self.bullet.is_fired {
            let bullet_location = self.location.clone();
            let target = Point2::new(   random::<f32>() * game_width,
                                        random::<f32>() * game_height);
            self.bullet.fire(bullet_location, target);
        }

        if self.is_at_destination() {
            let x = random::<f32>() * game_width;
            let y = random::<f32>() * game_height;

            self.destination = Point2::new(x, y);
        }

        self.step(dt);
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        set_color(context, self.color)?;
        circle(context, DrawMode::Line(1.0), self.location, self.radius, 1.0)
    }

    fn keep_in_arena(&mut self, arena_width: f32, arena_height: f32) -> GameResult<()> {
        if self.location.y < 0.0 {
            self.location.y = 0.0;
        } else if self.location.y > arena_height {
            self.location.y = arena_height;
        }

        if self.location.x < 0.0 {
            self.location.x = 0.0;
        } else if self.location.x > arena_width {
            self.location.x = arena_width;
        }

        Ok(())
    }

    fn is_at_destination(&self) -> bool {
        let difference = Point2::new(
            self.location.x - self.destination.x,
            self.location.y - self.destination.y
        );
        let distance = get_magnitude(difference);

        distance < 3f32
    }

    fn step(&mut self, dt: f32) {
        let direction = Point2::new(
            self.destination.x - self.location.x,
            self.destination.y - self.location.y
        );
        let mut normalized_direction = Point2::new(0f32, 0f32);

        if let Some(result) = normalize(direction) {
            normalized_direction = result;
        }
        
        let velocity = Point2::new(
            normalized_direction.x * self.speed,
            normalized_direction.y * self.speed
        );

        self.location.x += velocity.x * dt;
        self.location.y += velocity.y * dt;
    }
}

struct Bullet {
    location: Point2,
    velocity: Point2,
    size: f32,
    is_fired: bool,
    color: Color
}

impl Bullet {
    fn new() -> Bullet {
        let size = 5.0;
        let velocity = Point2::new(500.0, 0.0);
        let is_fired = false;
        let color = Color::new(1.0, 1.0, 1.0, 1.0);
        let location = Point2::new(-5.0, -5.0);

        Bullet {
            location,
            velocity,
            size,
            is_fired,
            color
        }
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        set_color(context, self.color)?;
        circle(context, DrawMode::Fill, self.location, self.size, 1.0)?;

        Ok(())
    }

    fn update(&mut self, _context: &mut Context, dt: f32, width: f32, height: f32) -> GameResult<()> {
        if self.is_fired {
            self.location.x += self.velocity.x * dt;
            self.location.y += self.velocity.y * dt;

            if self.is_off_screen(width, height) {
                self.is_fired = false;
            }
        }

        Ok(())
    }

    fn is_off_screen(&self, width: f32, height: f32) -> bool {
        self.location.y < 0.0 || self.location.x > width || self.location.y > height || self.location.x < 0.0
    }

    fn fire(&mut self, location: Point2, target: Point2) {
        let mut direction = Point2::new(target.x - location.x, target.y - location.y);

        if let Some(point) = normalize(direction) {
            direction = point;
        }

        direction.x *= 500f32;
        direction.y *= 500f32;

        self.velocity = direction;
        self.location = location;
        self.is_fired = true;
    }
}

fn get_magnitude(vector: Point2) -> f32 {
    let magnitude_squared = (vector.x * vector.x) + (vector.y * vector.y);

    magnitude_squared.sqrt()
}

fn normalize(vector: Point2) -> Option<Point2> {
    let magnitude = get_magnitude(vector);

    if magnitude > 0.0 {
        Some(Point2::new(
            vector.x / magnitude,
            vector.y / magnitude
        ))
    } else {
        None
    }
}

fn main() {
    let mut configuration_read = File::open("conf.toml").unwrap();

    let configuration = Conf::from_toml_file(&mut configuration_read).unwrap();
    let context = &mut Context::load_from_conf("random_walkers", "Brookzerker", configuration).unwrap();
    let (width, height) = graphics::get_size(context);
    let game_state = &mut GameState::new(width as f32, height as f32).unwrap();

    event::run(context, game_state).unwrap();
}
