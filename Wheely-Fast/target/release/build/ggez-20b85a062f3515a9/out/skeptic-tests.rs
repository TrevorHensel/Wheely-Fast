extern crate skeptic;
#[test] fn generativeart_sect_project_setup_line_16() {
    let s = &format!(r####"
{}"####, r####"use ggez::*;

struct State {
}

impl ggez::event::EventHandler for State {
  fn update(&mut self, _ctx: &mut Context) -> GameResult {
      Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult {
      graphics::present(ctx)?;
      Ok(())
  }
}

fn main() {
    let state = &mut State { };
    let cb = ggez::ContextBuilder::new("generative_art", "awesome_person");
    let (ref mut ctx, ref mut event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_ggez_graphics_line_73() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

struct State {{
    shapes: Vec<Shape>,
}}

impl State {{
    fn new(_ctx: &mut Context) -> GameResult<Self> {{
        let s = State {{ shapes: Vec::new() }};
        Ok(s)
    }}
}}

impl event::EventHandler for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        mint::Point2{x: 200.0, y: 300.0},
        100.0,
        0.1,
        graphics::WHITE,
    )?;
    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    graphics::present(ctx)?;
    Ok(())
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_ggez_graphics_line_110() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

struct State {{
    shapes: Vec<Shape>,
}}

impl State {{
    fn new(_ctx: &mut Context) -> GameResult<Self> {{
        let s = State {{ shapes: Vec::new() }};
        Ok(s)
    }}
}}

impl event::EventHandler for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let circle = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        mint::Point2{x: 200.0, y: 300.0},
        100.0,
        0.1,
        graphics::WHITE,
    )?;
    graphics::draw(ctx, &circle, graphics::DrawParam::default())?;
    graphics::present(ctx)?;
    Ok(())
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_ggez_graphics_line_133() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, _) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let mut shapes: Vec<Shape> = Vec::new();
    {}

    Ok(())
}}
"####, r####"graphics::Mesh::new_rectangle(
    ctx,
    graphics::DrawMode::fill(),
    graphics::Rect::new(500.0, 250.0, 200.0, 100.0),
    graphics::WHITE,
)?;
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_ggez_graphics_line_154() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

struct State {{
    shapes: Vec<Shape>,
}}

impl State {{
    fn new(_ctx: &mut Context) -> GameResult<Self> {{
        let s = State {{ shapes: Vec::new() }};
        Ok(s)
    }}
}}

impl event::EventHandler for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn draw(&mut self, ctx: &mut Context) -> GameResult {
    let rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(500.0, 250.0, 200.0, 100.0),
        graphics::WHITE,
    )?;
    graphics::draw(ctx, &rect, graphics::DrawParam::default())?;
    graphics::present(ctx)?;
    Ok(())
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_183() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

mod scope_hack {{  
    use super::*;

    pub enum Shape {{
        Circle(mint::Point2<f32>, f32),
        Rectangle(graphics::Rect),
    }}
}}

use self::scope_hack::*;

{}

fn main() {{

}}
"####, r####"struct State {
    shapes: Vec<Shape>,
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_194() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

mod scope_hack {{  
    use super::*;

    pub enum Shape {{
        Circle(mint::Point2<f32>, f32),
        Rectangle(graphics::Rect),
    }}
}}

use self::scope_hack::*;

{}

fn main() {{

}}
"####, r####"enum Shape {
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_203() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

struct State {{
    shapes: Vec<Shape>,
}}

impl event::EventHandler for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}
}}

{}

"####, r####"fn main() {
    let mut shapes = Vec::new();
    shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
        10.0,
        10.0,
        50.0,
        100.0,
    )));
    shapes.push(Shape::Circle(
        mint::Point2{x: 400.0, y: 40.0},
        30.0,
    ));
    let state = &mut State { shapes: shapes };
    let c = conf::Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("generative_art", "awesome_person")
        .conf(c)
        .build()
        .unwrap();
    event::run(ctx, event_loop, state).unwrap();
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_229() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

struct State {{
    shapes: Vec<Shape>,
}}

impl State {{
    fn new(_ctx: &mut Context) -> GameResult<Self> {{
        let s = State {{ shapes: Vec::new() }};
        Ok(s)
    }}
}}

impl event::EventHandler for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn draw(&mut self, ctx: &mut Context) -> GameResult {
    for shape in &self.shapes {
        // Make the shape...
        let mesh = match shape {
            &Shape::Rectangle(rect) => {
                Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, graphics::WHITE)?
            }
            &Shape::Circle(origin, radius) => {
                Mesh::new_circle(ctx, graphics::DrawMode::fill(), origin, radius, 0.1, graphics::WHITE)?
            }
        };

        // ...and then draw it.
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
    }
    graphics::present(ctx)?;
    Ok(())
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_268() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

mod scope_hack {{  
    use super::*;

    pub enum Shape {{
        Circle(mint::Point2<f32>, f32),
        Rectangle(graphics::Rect),
    }}
}}

use self::scope_hack::*;

{}

fn main() {{

}}
"####, r####"extern crate rand;
use rand::{thread_rng, Rng};
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_274() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, _) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let mut shapes: Vec<Shape> = Vec::new();
    {}

    Ok(())
}}
"####, r####"shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
    thread_rng().gen_range(0.0, 800.0),
    thread_rng().gen_range(0.0, 600.0),
    thread_rng().gen_range(0.0, 800.0),
    thread_rng().gen_range(0.0, 600.0),
)));
shapes.push(Shape::Circle(
    mint::Point2{
        x: thread_rng().gen_range(0.0, 800.0),
        y: thread_rng().gen_range(0.0, 600.0),
    },
    thread_rng().gen_range(0.0, 300.0),
));
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn generativeart_sect_random_shapes_line_295() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use rand::*;

enum Shape {{
    Circle(mint::Point2<f32>, f32),
    Rectangle(graphics::Rect),
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, _) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let mut shapes: Vec<Shape> = Vec::new();
    {}

    Ok(())
}}
"####, r####"let mut shapes = Vec::new();
for _ in 0..8 {
    if thread_rng().gen_range(0, 2) % 2 == 0 {
        shapes.push(Shape::Rectangle(ggez::graphics::Rect::new(
            thread_rng().gen_range(0.0, 800.0),
            thread_rng().gen_range(0.0, 600.0),
            thread_rng().gen_range(0.0, 800.0),
            thread_rng().gen_range(0.0, 600.0),
        )));
    } else {
        shapes.push(Shape::Circle(
            mint::Point2{
                x: thread_rng().gen_range(0.0, 800.0),
                y: thread_rng().gen_range(0.0, 600.0),
            },
            thread_rng().gen_range(0.0, 300.0),
        ));
    }
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn helloggez_sect_ggez_line_59() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

{}

fn main() {{

}}
"####, r####"use ggez::*;
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn helloggez_sect_ggez_line_83() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

{}

fn main() {{

}}
"####, r####"struct State {}

impl ggez::event::EventHandler for State {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
      Ok(())
  }
  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
      Ok(())
  }
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn helloggez_sect_ggez_line_108() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

struct State {{

}}

{}
"####, r####"pub fn main() {
    let state = &mut State { };
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn helloggez_sect_ggez_line_159() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use rand::*;
use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl EventHandler for State {{
    fn update(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}

    fn draw(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let state = &mut State {{ dt: Duration::default() }};
    {}

    Ok(())
}}
"####, r####"let c = conf::Conf::new();
let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
    .conf(c)
    .build()
    .unwrap();
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn helloggez_sect_ggez_line_174() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use rand::*;
use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl EventHandler for State {{
    fn update(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}

    fn draw(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let state = &mut State {{ dt: Duration::default() }};
    {}

    Ok(())
}}
"####, r####"event::run(ctx, event_loop, state).unwrap();
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn helloggez_sect_ggez_line_216() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

{}

fn main() {{

}}
"####, r####"struct State {
    dt: std::time::Duration,
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn helloggez_sect_ggez_line_224() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use rand::*;
use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl EventHandler for State {{
    fn update(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}

    fn draw(&mut self, _c: &mut Context) -> GameResult {{
        Ok(())
    }}
}}

pub fn main() -> GameResult {{
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("foo", "bar")
        .build()
        .unwrap();
    let state = &mut State {{ dt: Duration::default() }};
    {}

    Ok(())
}}
"####, r####"let state = &mut State { dt: std::time::Duration::new(0, 0) };
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn helloggez_sect_ggez_line_230() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;

use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl event::EventHandler for State {{
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn update(&mut self, ctx: &mut Context) -> GameResult {
    self.dt = timer::delta(ctx);
    Ok(())
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn helloggez_sect_ggez_line_238() {
    let s = &format!(r####"
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use ggez::*;
use ggez::graphics::*;
use ggez::event::*;
use std::time::Duration;

struct State {{
    pub dt: Duration,
}}

impl EventHandler for State {{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {{
        Ok(())
    }}

    {}
}}

pub fn main() -> GameResult {{
    Ok(())
}}
"####, r####"fn draw(&mut self, ctx: &mut Context) -> GameResult {
    println!("Hello ggez! dt = {}ns", self.dt.subsec_nanos());
    Ok(())
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

#[test] fn readme_sect_examples_line_103() {
    let s = &format!(r####"
{}"####, r####"use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new(&mut ctx);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

struct MyGame {
    // Your state here...
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        MyGame {
            // ...
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);
        // Draw code here...
        graphics::present(ctx)
    }
}
"####);
    skeptic::rt::compile_test(r#"/Users/Tavernetti/.cargo/registry/src/github.com-1ecc6299db9ec823/ggez-0.5.1"#, r#"/Users/Tavernetti/School/2019_Summer/CS410P/Wheely-Fast/Wheely-Fast/target/release/build/ggez-20b85a062f3515a9/out"#, r#"x86_64-apple-darwin"#, s);
}

