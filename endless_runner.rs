use ggez::{Context, GameResult, event, graphics};
use ggez::nalgebra as na;
use rand::Rng;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;

struct Player {
    rect: graphics::Rect,
    velocity: f32,
    jumping: bool,
}

struct Obstacle {
    rect: graphics::Rect,
    speed: f32,
}

struct GameState {
    player: Player,
    obstacles: Vec<Obstacle>,
    score: u32,
    background: graphics::Image,
}

impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let player = Player {
            rect: graphics::Rect::new(50.0, 300.0, 50.0, 50.0),
            velocity: 0.0,
            jumping: false,
        };
        let obstacles = vec![];
        let score = 0;
        let background = graphics::Image::new(ctx, "/background.png")?; // Load a background image

        Ok(GameState { player, obstacles, score, background })
    }

    fn update(&mut self) {
        // Gravity
        if self.player.jumping {
            self.player.velocity += 0.5; // Gravity effect
            self.player.rect.y += self.player.velocity;
            if self.player.rect.y >= 300.0 {
                self.player.rect.y = 300.0;
                self.player.jumping = false;
                self.player.velocity = 0.0;
            }
        }

        // Generate new obstacles
        if self.score % 100 == 0 {
            let mut rng = rand::thread_rng();
            let obstacle_height = rng.gen_range(30.0..100.0);
            self.obstacles.push(Obstacle {
                rect: graphics::Rect::new(SCREEN_WIDTH, SCREEN_HEIGHT - obstacle_height, 30.0, obstacle_height),
                speed: 5.0,
            });
        }

        // Move obstacles
        for obstacle in &mut self.obstacles {
            obstacle.rect.x -= obstacle.speed; // Move left
        }

        // Remove off-screen obstacles
        self.obstacles.retain(|obstacle| obstacle.rect.x > 0.0);
        self.score += 1; // Increment score
    }

    fn jump(&mut self) {
        if !self.player.jumping {
            self.player.jumping = true;
            self.player.velocity = -10.0; // Jump force
        }
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(135, 206, 235)); // Sky blue background

        // Draw background
        graphics::draw(ctx, &self.background, na::Point2::new(0.0, 0.0))?;

        // Draw player
        let player_rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.player.rect, graphics::Color::from_rgb(0, 255, 0))?;
        graphics::draw(ctx, &player_rect, na::Point2::new(0.0, 0.0))?;

        // Draw obstacles
        for obstacle in &self.obstacles {
            let obstacle_rect = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), obstacle.rect, graphics::Color::from_rgb(255, 0, 0))?;
            graphics::draw(ctx, &obstacle_rect, na::Point2::new(0.0, 0.0))?;
        }

        // Draw score
        let score_text = graphics::Text::new(format!("Score: {}", self.score));
        graphics::draw(ctx, &score_text, na::Point2::new(10.0, 10.0))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keycode: event::KeyCode, _keymod: event::Mod, _repeat: bool) {
        if keycode == event::KeyCode::Space {
            self.jump();
        }
    }
}

pub fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("endless_runner", "author")
        .build()?;
    let mut state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
