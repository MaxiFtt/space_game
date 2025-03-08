use std::path;

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::KeyCode;
use ggez::winit::event::KeyboardInput;
use ggez::{glam::*, Context, ContextBuilder, GameResult};

mod lerp;
use crate::lerp::Lerp;
fn main() -> GameResult {
    // Make a Context.
    let resource_path = path::PathBuf::from("./resources");
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Gordo Freedom")
        .add_resource_path(resource_path)
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = GameState::new(&mut ctx);

    // Run!
    event::run(ctx, event_loop, my_game.unwrap());
}
struct Entity {
    image: graphics::Image,
    pos: Vec2,
    rotation: f32,
    velocity: f32,
    damage: f32,
}

struct Player {
    image: graphics::Image,
    pos: Vec2,
    rotation: f32,
    velocity: Vec2,
}

struct GameState {
    player: Player,
    entities: Vec<Entity>,
    dt: std::time::Duration,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<GameState> {
        // Load/create resources such as images here.
        let player = Player {
            image: graphics::Image::from_path(ctx, "/PNG/playerShip1_blue.png")?,
            pos: Vec2::new(500.0, 500.0),
            rotation: 0.0,
            velocity: Vec2::new(0.0, 0.0),
        };
        let s = GameState {
            player,
            entities: vec![],
            dt: std::time::Duration::new(0, 0),
        };
        Ok(s)
    }
}
fn build_player(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.line(
        &[
            Vec2::new(342.0, 131.0),
            Vec2::new(400.0, 200.0),
            Vec2::new(410.0, 400.0),
            Vec2::new(220.0, 300.0),
            Vec2::new(200.0, 310.0),
        ],
        2.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    )?;

    mb.ellipse(
        graphics::DrawMode::fill(),
        Vec2::new(600.0, 200.0),
        50.0,
        120.0,
        1.0,
        Color::new(1.0, 1.0, 0.0, 1.0),
    )?;

    mb.circle(
        graphics::DrawMode::fill(),
        Vec2::new(600.0, 380.0),
        40.0,
        1.0,
        Color::new(1.0, 0.0, 1.0, 1.0),
    )?;

    Ok(graphics::Mesh::from_data(ctx, mb.build()))
}
fn handle_input(player: &mut Player, ctx: &mut Context, dt: std::time::Duration) -> () {
    if ctx.keyboard.pressed_keys().contains(&KeyCode::A) {
        player.velocity.x.lerp(-10.0, 2.0 * dt.as_secs_f32());
    }
    if ctx.keyboard.pressed_keys().contains(&KeyCode::D) {
        player.velocity.x.lerp(10.0, 2.0 * dt.as_secs_f32());
    }
    if ctx.keyboard.pressed_keys().contains(&KeyCode::W) {
        player.velocity.y.lerp(-10.0, 2.0 * dt.as_secs_f32());
    }
    if ctx.keyboard.pressed_keys().contains(&KeyCode::S) {
        player.velocity.y.lerp(10.0, 2.0 * dt.as_secs_f32());
    }
}
impl EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        while ctx.time.check_update_time(60) {
            self.player.pos.y += self.player.velocity.y;
            self.player.pos.x += self.player.velocity.x;
            handle_input(&mut self.player, ctx, self.dt);
            let is_x_axis_modified = if ctx
                .keyboard
                .pressed_keys()
                .iter()
                .filter(|x| **x == KeyCode::D || **x == KeyCode::A)
                .collect::<Vec<_>>()
                .is_empty()
            {
                false
            } else {
                true
            };
            let is_y_axis_modified = if ctx
                .keyboard
                .pressed_keys()
                .iter()
                .filter(|x| **x == KeyCode::W || **x == KeyCode::S)
                .collect::<Vec<_>>()
                .is_empty()
            {
                false
            } else {
                true
            };
            if !is_x_axis_modified {
                self.player
                    .velocity
                    .x
                    .lerp(0.0, 5.0 * self.dt.as_secs_f32())
            }
            if !is_y_axis_modified {
                self.player
                    .velocity
                    .y
                    .lerp(0.0, 5.0 * self.dt.as_secs_f32())
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        // Draw code here...
        canvas.draw(
            &self.player.image,
            graphics::DrawParam::new().dest(self.player.pos),
        );
        canvas.finish(ctx)
    }
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        repeated: bool,
    ) -> Result<(), ggez::GameError> {
        println!("{:?}", input.mods);
        Ok(())
    }
}
