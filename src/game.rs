use::piston_window::*;
use::piston_window::types::Color;

use rand::{thread_rng,Rng};


use crate::snake::{Direction, Snake};
use crate::draw::{draw_block,draw_rectangle};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const POISONFOOD_COLOR:Color = [0.00, 0.00, 0.80, 1.0];
const SPEEDBOOSTFOOD_COLOR:Color = [1.00, 0.00, 1.0, 1.0];
const SPEEDHINDERFOOD_COLOR: Color = [0.00, 1.00, 1.00, 1.0];

const BORDER_COLOR: Color=[0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color=[0.90, 0.00, 0.00, 0.5];

const SPEED_BOOST_FOOD:f64=0.04;
const SPEED_HINDER_FOOD:f64=0.02;
const DEFAULT_SPEED: f64=0.1;
const RESTART_TIME: f64=1.0;

#[derive(Copy, Clone,PartialEq)]
pub enum Food{
    Food,
    PosionFood,
    SpeedBoostFood,
    SpeedHinderFood,
}

pub struct Game{
    snake:Snake,
    food_exists:bool,
    foodtype:Food,
    food_x:i32,
    food_y:i32,
    width:i32,
    height:i32,
    game_over:bool,
    waiting_time:f64,
    speed_of_the_game:f64,
}
impl Game {
    pub fn new(width: i32, height: i32)->Game {
        Game {
            snake:Snake::new(2,2),
            
            food_exists: true,
            foodtype:Food::Food,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
            waiting_time: 0.0,
            speed_of_the_game:DEFAULT_SPEED,
        }
    }

    pub fn key_pressed(&mut self, key:Key){
        if self.game_over {
            return;
        }

        let dir=match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Right => Some(Direction::Right),
            Key::Left => Some(Direction::Left),
            _=>None
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con:&Context, g:&mut G2d) {
        self.snake.draw(con,g);

        if self.food_exists {
            match self.foodtype {
                Food::Food=>draw_block(FOOD_COLOR,self.food_x,self.food_y,con,g),
                Food::PosionFood=>draw_block(POISONFOOD_COLOR,self.food_x,self.food_y,con,g),
                Food::SpeedBoostFood=>draw_block(SPEEDBOOSTFOOD_COLOR,self.food_x,self.food_y,con,g),
                Food::SpeedHinderFood=>draw_block(SPEEDHINDERFOOD_COLOR,self.food_x,self.food_y,con,g),
            }
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height-1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width-1, 0, 1, self.height, con, g);

        if self.game_over{
            draw_rectangle(GAMEOVER_COLOR,0,0,self.width, self.height,con,g);
        }
    }

    pub fn update(&mut self,delta_time: f64){
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart(); 
            }
            return;
        }

        if self.snake.get_body_length()==0 {
            self.restart();
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > self.speed_of_the_game {
            self.update_snake(None);
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y== head_y {
            self.food_exists=false;
            match self.foodtype{
                Food::Food =>self.snake.restore_tail (),
                Food::PosionFood =>self.snake.remove_tail(),
                Food::SpeedBoostFood =>self.speed_of_the_game-=SPEED_BOOST_FOOD,
                Food::SpeedHinderFood =>{
                    self.speed_of_the_game+=SPEED_HINDER_FOOD;
                    if self.speed_of_the_game>DEFAULT_SPEED
                    {
                       self.speed_of_the_game=DEFAULT_SPEED;
                    }
                },
            }
            
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) ->bool {
        let (next_x, next_y) = self.snake.next_head(dir);

       if self.snake.overlap_tail(next_x,next_y){
            return false;
       }

        next_x > 0 && next_y > 0 && next_x<self.width-1 && next_y<self.height-1
    }

    fn add_food(&mut self) {
        let mut rng=thread_rng();

        let mut new_x = rng.gen_range(1..self.width-1);
        let mut new_y = rng.gen_range(1..self.width-1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width-1);
            new_y = rng.gen_range(1..self.width-1);
        }
        let new_type_food=rng.gen_range(1..=4);
        let new_type_of_food:Food;
        if new_type_food==1 {
            new_type_of_food=Food::Food;
            self.foodtype=new_type_of_food;
        }
        else if new_type_food==2 {
            new_type_of_food=Food::PosionFood;
            self.foodtype=new_type_of_food;
        }else if new_type_food==3 {
            new_type_of_food=Food::SpeedBoostFood;
            self.foodtype=new_type_of_food;
        }else{
            new_type_of_food=Food::SpeedHinderFood;
            self.foodtype=new_type_of_food;
        }

        self.food_x=new_x;
        self.food_y=new_y;
        self.food_exists=true;
         
    }

    pub fn update_snake(&mut self,dir: Option<Direction>) {
         if self.check_if_snake_alive(dir) {
              self.snake.move_forward(dir);
              self.check_eating();
        } else {
             self.game_over=true;
        }
        self.waiting_time=0.0;
    }

    fn restart(&mut self) {
        self.snake=Snake::new(2,2);
        self.waiting_time=0.0;
        self.foodtype=Food::Food;
        self.food_exists=true;
        self.food_x=6;
        self.food_y=4;
        self.game_over=false;
        self.speed_of_the_game=DEFAULT_SPEED;
    }


}