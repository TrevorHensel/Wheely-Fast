//Citations:
//
//GGEZ -
//We used this engine as it seemed simple enough to use and get our game up and running.
//We also used their basic game template from their website in order to start our project.
//https://github.com/ggez/ggez


use ggez::{graphics, Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Wheely_Fast", "Ryan Campbell, Trevor Hensel, Brooks Russel")
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
