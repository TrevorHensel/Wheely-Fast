//Trying to create a scrolling background
//using spritebatching in ggez

use ggez;

use ggez::event;
use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::timer;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct MainState {
    spritebatch: graphics::spritebatch::SpriteBatch,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "/background.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        let s = MainState { spritebatch: batch };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if timer::ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::delta(ctx));
            println!("Average FPS: {}", timer::fps(ctx));
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let time = (timer::duration_to_f64(timer::time_since_start(ctx)) * 1000.0) as u32;
        for x in 0..150 {
//            let mut y = 0.0;
            let p = graphics::DrawParam::new()
//                .dest(Point2::new((1500 / 2) as f32, (1500 / 2) as f32))
//                .dest(Point2::new(0.0, 0.0))
//                .dest(Point2::new((time / 4) as f32 + x,0.0))

//un-comment this for one scroll
                .dest(Point2::new(190.0, (x * -450) as f32))
                .scale(Vector2::new(1.0, 1.0,))
                .rotation(0.0);
            self.spritebatch.add(p);
        }
        let param = graphics::DrawParam::new()
//            .dest(Point2::new((1500 / 2) as f32, (1500 / 2) as f32))
            .dest(Point2::new(0.0, (time / 10) as f32))
            .scale(Vector2::new(1.0, 1.0,))
            .rotation(0.0)
            .offset(Point2::new(0.0, 0.0));

        graphics::draw(ctx, &self.spritebatch, param)?;
        self.spritebatch.clear();

        graphics::present(ctx)?;
        Ok(())
/*
        let time = (timer::duration_to_f64(timer::time_since_start(ctx)) * 1000.0) as u32;
        let cycle = 10_000;
        for x in 0..150 {
            for y in 0..150 {
                let x = x as f32;
                let y = y as f32;
                let p = graphics::DrawParam::new()
                    .dest(Point2::new(x * 10.0, y * 10.0))
                    .scale(Vector2::new(
                        ((time % cycle * 2) as f32 / cycle as f32 * 6.28)
                        .cos()
                        .abs()
                        * 0.0625,
                        ((time % cycle * 2) as f32 / cycle as f32 * 6.28)
                        .cos()
                        .abs()
                        * 0.0625,
                    ))
                    .rotation(-2.0 * ((time % cycle) as f32 / cycle as f32 * 6.28));
                self.spritebatch.add(p);
            }
        }
        let param = graphics::DrawParam::new()
            .dest(Point2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).cos() * 50.0 - 350.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin() * 50.0 - 450.0,
            ))
            .scale(Vector2::new(
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
                ((time % cycle) as f32 / cycle as f32 * 6.28).sin().abs() * 2.0 + 1.0,
            ))
            .rotation((time % cycle) as f32 / cycle as f32 * 6.28)
            .offset(Point2::new(750.0, 750.0));
        graphics::draw(ctx, &self.spritebatch, param)?;
        self.spritebatch.clear();

        graphics::present(ctx)?;
        Ok(())
*/
    }
}

pub fn main() -> GameResult {
    if cfg!(debug_assertions) && env::var("yes_i_really_want_debug_mode").is_err() {
        eprintln!(
            "Note: Release mode will improve performance greatly.\n     \
             e.g use 'cargo run --example spritebatch -- release'"
        );
    }

    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("spritebatch", "ggez").add_resource_path(resource_dir);
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
