extern crate ggez;
extern crate rand;

use ggez::event::EventHandler;
use ggez::{GameResult, Context, event, graphics};
use ggez::conf::Conf;
use ggez::graphics::{circle, DrawMode, present, Point2, Color, set_color};

use rand::random;

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

        walkers.push(RandomWalker::new(width, height, "red")?);
        walkers.push(RandomWalker::new(width, height, "blue")?);
        walkers.push(RandomWalker::new(width, height, "green")?);

        Ok(GameState {
            walkers,
            width,
            height
        })
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        for walker in &mut self.walkers {
            walker.step()?;
            walker.keep_in_arena(self.width, self.height)?;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        for walker in &mut self.walkers {
            walker.draw(context)?;
        }

        present(context);
        Ok(())
    }
}

struct RandomWalker {
    location: Point2,
    radius: f32,
    color: Color
}

impl RandomWalker {
    fn new(width: f32, height: f32, color_string: &str) -> GameResult<RandomWalker> {
        let x = random::<f32>() * width;
        let y = random::<f32>() * height;
        let color = match color_string {
            "red" => Color::new(1.0, 0.0, 0.0, 1.0),
            "green" => Color::new(0.0, 1.0, 0.0, 1.0),
            "blue" => Color::new(0.0, 0.0, 1.0, 1.0),
            &_ => panic!("pass in valid color only") // todo pass proper error back
        };

        Ok(RandomWalker {
            location: Point2::new(x, y),
            radius: 1.0,
            color
        })
    }

    fn step(&mut self) -> GameResult<()> {
        let random_number = random::<f32>();
        
        if random_number > 0.75 {
            self.location.y -= 1.0;
        } else if random_number > 0.5 {
            self.location.x += 1.0;
        } else if random_number > 0.25 {
            self.location.y += 1.0;
        } else {
            self.location.x -= 1.0;
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        set_color(context, self.color)?;
        circle(context, DrawMode::Fill, self.location, self.radius, 1.0)
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
}

fn main() {
    let configuration = Conf::new();
    let context = &mut Context::load_from_conf("random_walkers", "Brookzerker", configuration).unwrap();
    let (width, height) = graphics::get_size(context);
    let game_state = &mut GameState::new(width as f32, height as f32).unwrap();

    event::run(context, game_state).unwrap();
}
