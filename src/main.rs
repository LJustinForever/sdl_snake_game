use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;
use std::{u32, usize};
use sdl2::rect::Rect; 
use rand::prelude::*;

const GAME_TITLE : &str = "Snakey Game";

const SCREEN_WIDHT : u32 = 600;
const SCREEN_HEIGHT : u32 = 800;

const PLAYER_STEP : u8 = 10;

const RECT_SIZE : i32 = 20;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
struct Point{
    x : i32,
    y : i32
}

impl Point {
    fn is_intersected(&self, point: &Point, rect_size: i32) -> bool{
        let mut result = false;
        if (self.x >= (point.x - rect_size))
        && (self.y >= (point.y - rect_size))
        && (self.x <= (point.x + rect_size))
        && (self.y <= (point.y + rect_size)){ 
            result = true;
        }
       result 
    }
}

#[derive(Debug)]
struct Food{
    pos: Point,
    is_spawned: bool
}

impl Food {
    fn new() -> Self{
        Food{
            pos: Point{x: 30, y: 30},
            is_spawned: true
        }
    }

    fn spawn(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){ 
        if !self.is_spawned {
            self.pos = Point{x: rand::thread_rng().gen_range(2..=(SCREEN_WIDHT as i32 - 50)/10)* 10,
                            y: rand::thread_rng().gen_range(2..=(SCREEN_HEIGHT as i32 - 50)/10) * 10};
            self.is_spawned = true;
            //println!("FOOD: {:?}", (self.pos.x, self.pos.y))
        }
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        let rect = Rect::new(self.pos.x, self.pos.y, RECT_SIZE as u32, RECT_SIZE as u32);
        canvas.fill_rect(rect).expect("Error drawing spawning food");
    }
}
 
#[derive(Debug)]
struct Border{
    position: Point,
    pub width: u32,
    pub height: u32
}

impl Border {
    fn new() -> Self{
        Border{
            position: Point{x: 0, y: 0},
            width: SCREEN_WIDHT,
            height: SCREEN_HEIGHT
        }
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let rect = Rect::new(self.position.x, self.position.y, self.width - 10, self.height - 10);
        let _ = canvas.draw_rect(rect);
    }
}

//Snake
#[derive(Debug)]
struct Player {
    position : Point,
    previous_position: Point,
    current_direction: Direction,
    length : u32,
    speed : u32,
    tail_positions: Vec<Point>
}

impl Player {
    fn new(point: Point) -> Self{
        Player{
            position: point,
            previous_position: point,
            current_direction: Direction::Up,
            length: 1,
            speed: 10,
            tail_positions: Vec::new()
        }
    }

    fn update_position(&mut self){
        self.previous_position = self.position;
        match self.current_direction{
            Direction::Up => self.position.y -= PLAYER_STEP as i32,
            Direction::Down => self.position.y += PLAYER_STEP as i32,
            Direction::Left => self.position.x -= PLAYER_STEP as i32,
            Direction::Right => self.position.x += PLAYER_STEP as i32
        }
    }

    fn draw(&mut self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.update_position();
        let player_rect = Rect::new(self.position.x, self.position.y, RECT_SIZE as u32, RECT_SIZE as u32);
        canvas.fill_rect(player_rect).expect("Error drawing player");

        for i in 0..self.length{
            if self.length == 1 {
                break;
            }
            //Todo move this to eating 
            if self.length > self.tail_positions.len() as u32 {
                if self.tail_positions.len() == 0 {
                    self.tail_positions.push(self.position);
                }
                else{
                    let last_tail = self.tail_positions.last().copied().expect("Unable to copy tail pos");
                    self.tail_positions.push(last_tail);
                }
            }
            let index = i as usize;
            let temp = self.tail_positions[index];
            self.tail_positions[index] = self.previous_position;
            self.previous_position = temp;

            let tail_rect = Rect::new(self.tail_positions[i as usize].x, self.tail_positions[i as usize].y, RECT_SIZE as u32, RECT_SIZE as u32);
            canvas.fill_rect(tail_rect).expect("Error drawing tail");
        }
    }

    fn check_hit(&self, border: &Border) -> bool{
        let mut result = false;
        //println!("Player: {:?}", (self.position.x, self.position.y));
        if (self.position.x + RECT_SIZE as i32 >= border.width as i32)
            || (self.position.y + RECT_SIZE as i32 >= border.height as i32)
            || (self.position.x as i32 <= 0)
            || (self.position.y as i32 <= 0){
                result = true;
            }
        
        // let mut i = 0;
        // for item in &self.tail_positions {
        //     if i == 0 {
        //         continue;
        //     }
        //     if self.position.is_intersected(&item, RECT_SIZE/2) {
        //         result = true;
        //         break;
        //     }
        //     i += 1;
        // }

        result
    }

    fn check_food(&self, food: &mut Food) -> bool {
        let mut result = false;
        if self.position.is_intersected(&food.pos, RECT_SIZE/2){ 
            food.is_spawned = false;
            result = true;
        }
        result
    }

    fn eat_food(&mut self){
        self.length += 1;
        self.speed += 1;
    }
}

fn event_handler(event : Event,player: &mut Player) -> bool{
    let result = match event{
        Event::Quit { .. } |
        Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>{
            true
        },
        Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
            if player.current_direction != Direction::Down {
                player.current_direction = Direction::Up;
            }
            return false;
        },
        Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
            if player.current_direction != Direction::Up {
                player.current_direction = Direction::Down;
            }
            return false;
        },
        Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
            if player.current_direction != Direction::Left{
                player.current_direction = Direction::Right;
            }
            return false;
        },
        Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
            if player.current_direction != Direction::Right {
                player.current_direction = Direction::Left;
            }
            return false;
        },
        Event::KeyDown {keycode: Some(Keycode::B), ..} => {
            player.length += 1;
            return false;
        }
        _=>false
    };
    result
}



fn main() -> Result<(), String>{

    //Init 
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(GAME_TITLE, SCREEN_WIDHT, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("Error initialising video subsystem");
    
    let mut canvas = window.into_canvas().build()
        .expect("Error initialising canvas");

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    //Objects
    
    let mut player = Player::new(Point{ x: SCREEN_WIDHT as i32 / 2, y: SCREEN_HEIGHT as i32 / 2});
    let border = Border::new();
    let mut food = Food::new();
    let mut event_pump = sdl_context.event_pump()?;

    //Game loop
    'running: loop{
        for event in event_pump.poll_iter(){
            if event_handler(event, &mut player){
              break 'running;
            } 
        }

        //Update
        canvas.set_draw_color(Color::RGB(0, 0, 0)); 
        canvas.clear();
    
        if player.check_hit(&border){
            break 'running;
        }

        if player.check_food(&mut food) {
            player.eat_food();
        }

        //Draw Objects
        player.draw(&mut canvas);
        border.draw(&mut canvas);
        food.spawn(&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / player.speed));
    }
    Ok(())
}
