extern crate queues;
extern crate rand;

use ggez;
use ggez::event::{KeyCode, KeyMods};
use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::timer;
use ggez::{event, Context, GameResult};
use std::env;
use std::path;

use std::time::{Duration, Instant};

use queues::*;
use rand::Rng;

//30x30 grid, not sure how big it should be right now this is just for testing. For not it will be 25x50
const GRID_SIZE: (i16, i16) = (25, 40);
//The number of pixels in each cell on the grid, 17x17
const GRID_CELL_SIZE: (i16, i16) = (17, 17);

//The 'x' position for each lane, for each barrier image to appear center in its' respective lane
const LANE_1: f32 = 68.0;
const LANE_2: f32 = 174.0;
const LANE_3: f32 = 280.0;

//An arbitraury number that changes the speed of the background and barriers to make the game more difficult
//4-10 are good starting points
const DIFFICULTY: u32 = 7;

//Distance inbetween each barrier on the screen
//must be negative
//-200 is a good starting point

//***NOT IMPLEMENTED YET DON'T CHANGE***
const BARRIER_DISTANCE: i32 = -200;
//***NOT IMPLEMENTED YET DON'T CHANGE***

//Controls how fast the background and barriers speed up the further the player gets into the game
const SPEEDUP: f32 = 0.0000025;

//size of the game screen
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);

//Determines how quickly the game should update ,dont want the car to move to quickly across the screen so we can determine,
//the distance it moves every frame.
const UPDATES_PER_SECOND: f32 = 16.0;
const MS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;

//The barriers can not be generated in the same lane over and over again.
pub fn get_lane(last: i16) -> (f32, i16) {
    let mut rng = rand::thread_rng();
    let mut x: i16 = rng.gen_range(0, 3);
    while x == last {
        x = rng.gen_range(0, 3);
    }

    if x == 0 {
        return (LANE_1, 0);
    } else if x == 1 {
        return (LANE_2, 1);
    } else if x == 2 {
        return (LANE_3, 2);
    } else {
        (0.0, 4)
    }
}

//GameImages is used to load in the images used for certain parts of the game like the car.
struct GameImages {
    car_image: graphics::Image,
    font: graphics::Font,
    start_img: graphics::Image,
}

impl GameImages {
    fn new(ctx: &mut Context) -> GameResult<GameImages> {
        let car_image = graphics::Image::new(ctx, "/Car.png")?;
        let font = graphics::Font::new(ctx, "/CommodorePixelized.ttf")?;
        let start_img = graphics::Image::new(ctx, "/Start_Button.png").unwrap();

        Ok(GameImages { car_image, font, start_img})
    }
}

//The directions that the car can go
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    //Up is used to stop the car if no direction is given or if you hit the sides of the game.
    Up,
}

impl Direction {
    pub fn from_keycode(key: KeyCode) -> Option<Direction> {
        match key {
            KeyCode::Left => Some(Direction::Left),
            KeyCode::Right => Some(Direction::Right),
            KeyCode::Up => Some(Direction::Up),
            _ => None,
        }
    }
}

//The location of where the car is on the game grid. used for staying in the lane and crossing finish line
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridLocation {
    //Might only need an x value for now since our car only moves left and right.
    x: i16,
    y: i16,
}

impl GridLocation {
    //used to create new grid location easier when updating the position and initalizing the position.
    //I am not sure if we need the y-axis since we do not move up and down. We probably don't.
    pub fn new(x: i16, y: i16) -> Self {
        GridLocation { x, y }
    }

    //A helper function that takes a grid position and returns a new one after making a move in a spoecific direction.
    pub fn new_move(pos: GridLocation, dir: Direction) -> Self {
        match dir {
            //These directions are based of the game screen pixels and are chosen to fit out image.
            //Made more changes to the distance the car moves left and right to help with collsion.
            //The car moves to the middle of each lane so it hits the barrier head on instead of
            //the sides. This is because it is hard to implement collison for the sides of the
            //barrier.
            Direction::Left => GridLocation::new(pos.x - 108, pos.y),
            Direction::Right => GridLocation::new(pos.x + 108, pos.y),
            //The up direction is used to stop the car from moving. I want to look into a way for it to stop moving on key release.GridLocation
            //maybe it will I have to test
            Direction::Up => GridLocation::new(pos.x, pos.y),
        }
    }
}

//This will filla grid cell on our game board with the cars location. They can be used for other
//things as well that need locations on the board. We were thinking about using for collision but
//went another direction.
impl From<GridLocation> for graphics::Rect {
    fn from(pos: GridLocation) -> Self {
        graphics::Rect::new_i32(
            pos.x as i32 * GRID_CELL_SIZE.0 as i32,
            pos.y as i32 * GRID_CELL_SIZE.1 as i32,
            GRID_CELL_SIZE.0 as i32,
            GRID_CELL_SIZE.1 as i32,
        )
    }
}

//Helps us convert from i16 to GridLocation
impl From<(i16, i16)> for GridLocation {
    fn from(pos: (i16, i16)) -> Self {
        GridLocation { x: pos.0, y: pos.1 }
    }
}

//The car object that will be placed on the grid and manuevered.
struct Car {
    car: GridLocation,
    dir: Direction,
    next_dir: Option<Direction>,
}

impl Car {
    pub fn new(pos: GridLocation) -> Self {
        Car {
            //Initial car position, not moving in any direction.
            car: GridLocation::new(pos.x, pos.y),
            dir: Direction::Up,
            next_dir: None,
        }
    }

    //need to implement update function that will be used every time that the game needs to be updated.
    fn update(&mut self) {
        //update the direction from the key board input
        if self.next_dir.is_some() {
            self.dir = self.next_dir.unwrap();
            //these two if statments bind the car from going off the road. Must be changed if car
            //movement from left to right is change so the car stays on road.
            if self.car.x == 76 && self.dir == Direction::Left {
                self.dir = Direction::Up;
            }
            if self.car.x == 292 && self.dir == Direction::Right {
                self.dir = Direction::Up;
            }
            //if I change this to Up does the car stop?
            self.next_dir = None;
        } else {
            //makes it so the car stops after moving 1 cell in the direction of the last key press.
            self.dir = Direction::Up;
        };
        //Give the car a new position and direction
        let new_car_pos = GridLocation::new_move(self.car, self.dir);
        let new_car = GridLocation::new(new_car_pos.x, new_car_pos.y);

        self.car = new_car;

        //must eventually check for collisions here as well. maybe runnning of road too?
    }
    //Draw car
    fn draw(&self, ctx: &mut Context, pic: &mut GameImages) -> GameResult<()> {
        let image = &pic.car_image;
        let pos = self.car;
        let drawparams = graphics::DrawParam::new().dest(Point2::new(pos.x as f32, pos.y as f32));

        graphics::draw(ctx, image, drawparams)
    }
}

//Used to determine if the game is running/ should still be running.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlayState {
    Start,
    Play,
    End,
}

//MainState creates the games variables.
struct MainState {
    pics: GameImages,
    road: graphics::spritebatch::SpriteBatch, // Sprite batch of the road background image
    car: Car,
    barrier: graphics::spritebatch::SpriteBatch, // Sprite batch of the barrier image
    score: u128,
    start_time: u128, // Time when player begins play
    last_update: Instant,
    play: PlayState, // false means menu, true means gameplay
    next_barrier_lane: i16,
    lane_queue: Queue<f32>,
}

//add levels, score, stop the car from going off screen
impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        //Initializing all the variables of MainState when a new object is created
        let pics = GameImages::new(ctx)?;
        let background = graphics::Image::new(ctx, "/Background.png").unwrap();
        let background_batch = graphics::spritebatch::SpriteBatch::new(background);
        //Put car in the middle bottom section of screen or the cars initial location on the screen.
        let car_pos = (
            ((GRID_SIZE.0 * GRID_CELL_SIZE.0) / 2) - 28,
            (GRID_SIZE.1 * GRID_CELL_SIZE.1) - 100,
        )
            .into();
        let barrier_img = graphics::Image::new(ctx, "/Barrier.png").unwrap();
        let blockage = graphics::spritebatch::SpriteBatch::new(barrier_img);
        let q: Queue<f32> = queue![];

        let mut s = MainState {
            pics,
            road: background_batch,
            car: Car::new(car_pos),
            barrier: blockage,
            score: 0,
            start_time: 0,
            last_update: Instant::now(),
            play: PlayState::Start,
            next_barrier_lane: 0,
            lane_queue: q,
        };

        //This generates 450 barriers that are each 'BARRIER_DISTANCE' away from each other and loads them
        //into the barrier variable of MainState
        let mut test_last = 4;
        for x in 0..450 {
            let (i, _last) = get_lane(test_last);
            test_last = _last;
            s.lane_queue.add(i);
            //Generate a barrier every 'BARRIER_DISTANCE' pixels apart, where x = the nth barrier
            let j = graphics::DrawParam::new()
                .dest(Point2::new(i, (x * BARRIER_DISTANCE) as f32 + 50.0))
                .scale(Vector2::new(1.0, 1.0))
                .rotation(0.0);
            s.barrier.add(j);
        }

        Ok(s)
    }
}

//implements the EventHandler for the GameState. The event handler is the main loop for the main
//state. Any time something needs to be updated the event handler will call those functions.
impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.play == PlayState::Play {
            //check to see if enough time has passed so we can update again.
            if Instant::now() - self.last_update >= Duration::from_millis(MS_PER_UPDATE) {
                self.car.update();
                self.last_update = Instant::now();

                //Collision Detection
                let time_x =
                    (timer::duration_to_f64(timer::time_since_start(_ctx)) * 1000.0) as u32;
                let speedup_calculation =
                    (((time_x - self.start_time as u32).pow(2) as f32) * SPEEDUP) as u32;
                let offset_distance = self.start_time as u32 / DIFFICULTY;
                let y_position =
                    ((time_x / DIFFICULTY) - offset_distance + speedup_calculation) as f32;

                let temp = y_position % 200.0;
                println!("{}", temp);
                if y_position > 650.0 && temp > 100.0 && temp < 155.0 && self.next_barrier_lane == 1
                {
                    self.lane_queue.remove();
                    self.next_barrier_lane = 0;
                }
                if y_position > 450.0 && temp > 25.0 && temp < 65.0 && self.next_barrier_lane == 0 {
                    self.next_barrier_lane = 1;
                }

                //if temp < 13 this is after the car passes the barrier if the value of temp is
                //less than 13 then it hit the top of the barrier as it was passing.
                if temp < 10.0 || temp > 120.0 {
                    if y_position > 500.0 {
                        let x_pos = self.car.car.x as f32;
                        let lane = self.lane_queue.peek().unwrap();
                        if (lane - x_pos) < 15.0 && (lane - x_pos) > -35.0 {
                            self.play = PlayState::End;
                        }
                    }
                }
                //End Collision Detection

                //update score
                let time = timer::time_since_start(_ctx).as_millis();
                self.score = (time - self.start_time) / 64;
            }
        }
        Ok(())
    }

    //render the game
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let pics = &mut self.pics;
        //clear screen, can make the screens background a specific color here.
        graphics::clear(ctx, graphics::BLACK.into());

        //current time in miliseconds
        let time = (timer::duration_to_f64(timer::time_since_start(ctx)) * 1000.0) as u32;

        //generates 150 road images and stiches them together to create a scrolling background effect so the user
        //thinks the car is driving on a road
        for x in 0..150 {
            let p = graphics::DrawParam::new()
                //the y postion was calculated to get the images to stich together seemlessly
                .dest(Point2::new(0.0, ((x * -450) + 600) as f32))
                .scale(Vector2::new(1.0, 1.0))
                .rotation(0.0);
            self.road.add(p);
        }
        let mut speedup_calc: u32 = 0;
        if self.play == PlayState::Play {
            speedup_calc = ((time - self.start_time as u32).pow(2) as f32 * SPEEDUP) as u32;
        }
        let param = graphics::DrawParam::new()
            .dest(Point2::new(
                0.0,
                ((time / DIFFICULTY) + speedup_calc) as f32,
            ))
            .scale(Vector2::new(1.0, 1.0))
            .rotation(0.0)
            .offset(Point2::new(0.0, 0.0));

        graphics::draw(ctx, &self.road, param)?;
        self.road.clear();

        //if the game isnt started display the start button
        if self.play == PlayState::Start {
            let start_dest = Point2::new(SCREEN_SIZE.0 / 4.0, SCREEN_SIZE.1 / 2.0);
            graphics::draw(
                ctx,
                &pics.start_img,
                graphics::DrawParam::default().dest(start_dest),
            )?;
            //draw exit instructions
            let startmsg_dest = Point2::new(SCREEN_SIZE.0 / 4.0, SCREEN_SIZE.1 * 0.7);
            let startmsg_display = graphics::Text::new(("Press return", pics.font, 20.0));
            graphics::draw(
                ctx,
                &startmsg_display,
                (startmsg_dest, 0.0, graphics::Color::new(0.0, 0.0, 1.0, 1.0)),
            )?;
        }
        //else start generating barriers on the screen
        else if self.play == PlayState::End {
            //draw score
            let score_dest = Point2::new(SCREEN_SIZE.0 / 5.0, SCREEN_SIZE.1 / 2.0);
            let score_str = format!("Score: {}", self.score);
            let score_display = graphics::Text::new((score_str, pics.font, 34.0));
            graphics::draw(
                ctx,
                &score_display,
                (score_dest, 0.0, graphics::Color::new(1.0, 0.0, 0.0, 1.0)),
            )?; //red
                //draw exit instructions
            let exit_dest = Point2::new(SCREEN_SIZE.0 / 5.0, SCREEN_SIZE.1 * 0.6);
            let exit_display = graphics::Text::new(("Press esc to quit", pics.font, 18.0));
            graphics::draw(
                ctx,
                &exit_display,
                (exit_dest, 0.0, graphics::Color::new(0.0, 1.0, 0.0, 1.0)),
            )?;
        } else {
            let speedup_calculation =
                (((time - self.start_time as u32).pow(2) as f32) * SPEEDUP) as u32;
            let offset_distance = self.start_time as u32 / DIFFICULTY;
            let param2 = graphics::DrawParam::new()
                .dest(Point2::new(
                    0.0,
                    (((time / DIFFICULTY) - offset_distance) + speedup_calculation) as f32,
                ))
                .scale(Vector2::new(1.0, 1.0))
                .rotation(0.0)
                .offset(Point2::new(0.0, 0.0));

            graphics::draw(ctx, &self.barrier, param2)?;
            //draw score
            let score_dest = Point2::new(SCREEN_SIZE.0 / 8.0, 16.0);
            let score_str = format!("Score: {}", self.score);
            let score_display = graphics::Text::new((score_str, pics.font, 30.0));
            graphics::draw(
                ctx,
                &score_display,
                (score_dest, 0.0, graphics::Color::new(1.0, 0.0, 0.0, 1.0)),
            )?; //red
        }

        //draw car
        self.car.draw(ctx, pics)?;

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    //Key down event watches for any key presses and then updates those for where the car should
    //go.
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        if let Some(dir) = Direction::from_keycode(keycode) {
            // ensures direction is not changed  unless in play mode
            // this way the car stays in place even if arrow key is pressed before return
            if self.play == PlayState::Play {
                //just make the direction for the next left or right input the same as
                self.car.next_dir = Some(dir);
            }
        } else {
            match keycode {
                KeyCode::Return => {
                    // press return to start game
                    if self.play == PlayState::Start {
                        self.play = PlayState::Play;
                        self.start_time = timer::time_since_start(_ctx).as_millis();
                    }
                }
                KeyCode::Escape => {
                    // quit app by pressing escape key
                    if self.play == PlayState::Start || self.play == PlayState::End {
                        event::quit(_ctx);
                    } else {
                        self.play = PlayState::End;
                    }
                }
                _ => (), // do nothing
            }
        }
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("Racing Game", "Ryan Campbell")
        .window_setup(ggez::conf::WindowSetup::default().title("Racing"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .add_resource_path(resource_dir)
        .build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, state)
}
