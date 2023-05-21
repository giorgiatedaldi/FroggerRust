use core::num;
use std::any::Any;
use std::cmp::{min, max};

use crate::{actor::*, log, rand};
use crate::rand::*;

pub struct Raft {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    type_trunk: i32,
}
impl Raft {
    pub fn new(pos: Pt, speed: i32, type_trunk: i32) -> Raft {
        Raft{pos: pos, step: pt(1, 0), size: pt(if type_trunk == 0 { 64 } else { 96 } , 32), speed: speed, type_trunk: type_trunk}
    }
}
impl Actor for Raft {
    fn act(&mut self, arena: &mut ArenaStatus) {
        // for other in arena.collisions() {
        //     if let Some(_) = other.as_any().downcast_ref::<Ghost>() {
        //     } else {
        //         let diff = self.pos - other.pos();
        //         self.step.x = self.speed * if diff.x > 0 { 1 } else { -1 };
        //         self.step.y = self.speed * if diff.y > 0 { 1 } else { -1 };
        //     }
        // }
        if self.pos.x < -300 { self.pos.x = arena.size().x + 300 }
        if self.pos.x > arena.size().x + 300 { self.pos.x = -300 }
        //if tl.y < 0 { self.step.y = self.speed; }
        //if br.x > 0 { self.step.x = -self.speed; }
        //if br.y > 0 { self.step.y = -self.speed; }
        self.step.x = self.speed;
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> { 
        Some(pt(if self.type_trunk == 0 { 224 } else { 192 } , 96))
    }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
    fn speed(&self) -> i32 { self.speed }
}

pub struct Turtle {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    under_water: i32,
    change_sprite: i32,
    sprite_fps: i32,
    sprite_tick: i32,
}
impl Turtle {
    pub fn new(pos: Pt, speed: i32, under_water: i32, sprite_tick: i32) -> Turtle {
        Turtle{pos: pos, step: pt(1, 0), size: pt(32 , 32), speed: speed, under_water: under_water, change_sprite: 0, sprite_fps: 20, sprite_tick: sprite_tick}
    }
}
impl Actor for Turtle {
    fn act(&mut self, arena: &mut ArenaStatus) {
        self.change_sprite += 1;
        if self.under_water == 1  && self.change_sprite > (self.sprite_tick + self.sprite_fps * 6) {
            self.change_sprite = self.sprite_tick; 
        }
        else if self.under_water == 0 && self.change_sprite >= (self.sprite_tick + self.sprite_fps * 3) {
            self.change_sprite = self.sprite_tick;
        }   
        
        // for other in arena.collisions() {
        //     if let Some(_) = other.as_any().downcast_ref::<Ghost>() {
        //     } else {
        //         let diff = self.pos - other.pos();
        //         self.step.x = self.speed * if diff.x > 0 { 1 } else { -1 };
        //         self.step.y = self.speed * if diff.y > 0 { 1 } else { -1 };
        //     }
        // }
        if self.pos.x < -300 { self.pos.x = arena.size().x + 300 }
        if self.pos.x > arena.size().x + 300 { self.pos.x = -300 }
        //if tl.y < 0 { self.step.y = self.speed; }
        //if br.x > 0 { self.step.x = -self.speed; }
        //if br.y > 0 { self.step.y = -self.speed; }
        self.step.x = self.speed;
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> { 
        if self.change_sprite >= self.sprite_tick && self.change_sprite < (self.sprite_fps + self.sprite_tick) {
            Some(pt(256, 128))
        }
        else if self.change_sprite >= (self.sprite_fps + self.sprite_tick) && self.change_sprite < (self.sprite_fps * 2 + self.sprite_tick) {
            Some(pt(224, 128))
        }
        else if self.change_sprite >= (self.sprite_fps * 2 + self.sprite_tick) && self.change_sprite < (self.sprite_fps * 3 +  self.sprite_tick) {
            Some(pt(192, 128))
        }
        else if self.change_sprite >= (self.sprite_fps * 3 + self.sprite_tick) && self.change_sprite < (self.sprite_fps * 4 + self.sprite_tick) {
            Some(pt(192, 160))
        }
        else if self.change_sprite >= (self.sprite_fps * 4 + self.sprite_tick) && self.change_sprite < (self.sprite_fps * 5 + self.sprite_tick) {
            Some(pt(224, 160))
        }
        else {
            None
        }
    }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
    fn speed(&self) -> i32 { self.speed }
}

pub struct Vehicle {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    car: bool,
    type_car: i32,
}
impl Vehicle {
    pub fn new(pos: Pt, car: bool, speed: i32) -> Vehicle {
        Vehicle{pos: pos, step: pt(1, 0), size: pt(if car { 32 } else { 64 }, 32), speed: speed, car:car, type_car: randint(6, 9)}
    }
}
impl Actor for Vehicle {
    fn act(&mut self, arena: &mut ArenaStatus) {
        // for other in arena.collisions() {
        //     if let Some(_) = other.as_any().downcast_ref::<Ghost>() {
        //     } else {
        //         let diff = self.pos - other.pos();
        //         self.step.x = self.speed * if diff.x > 0 { 1 } else { -1 };
        //         self.step.y = self.speed * if diff.y > 0 { 1 } else { -1 };
        //     }
        // }
        //let tl = self.pos + self.step;  // top-left
        if self.pos.x < -300 { self.pos.x = arena.size().x + 300 }
        if self.pos.x > arena.size().x + 300 { self.pos.x = -300 }
        //if tl.y < 0 { self.step.y = self.speed; }
        //if br.x > 0 { self.step.x = -self.speed; }
        //if br.y > 0 { self.step.y = -self.speed; }
        self.step.x = self.speed;
        self.pos = self.pos + self.step;
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> { 
        //let mut car_type = 0;
        if self.car {
            Some(pt(self.type_car*32, if self.speed >= 0 && self.type_car == 8 { 32 } else if self.speed < 0 && self.type_car == 8 { 0 } else if self.speed >= 0 { 0 } else { 32 } )) 
        }
        else
        {
            Some(pt(if self.speed >= 0 { 256 } else { 192 }, 64))
        }
        
    }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
    fn speed(&self) -> i32 { self.speed }
}


pub struct Water {
    pos: Pt,
    size: Pt
}
impl Water {
    pub fn new(pos: Pt) -> Water {
        Water{pos: pos, size: pt(704, 192)}
    }
}
impl Actor for Water {
    fn act(&mut self, _arena: &mut ArenaStatus) {
    }
    fn sprite(&self) -> Option<Pt> { 
        None
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
    fn speed(&self) -> i32 { 0}
}


pub struct WinBox {
    pos: Pt,
    size: Pt,
    occupied: bool,
    number: i32,
}
impl WinBox {
    pub fn new(pos: Pt, number: i32) -> WinBox {
        WinBox{pos: pos, size: pt (32, 32), occupied: false, number: number}
    }
    pub fn get_number(&self) -> i32 { self.number }
}
impl Actor for WinBox {
    fn act(&mut self, arena: &mut ArenaStatus) {
        for other in arena.collisions() {
            if let Some(_) = other.as_any().downcast_ref::<Frog>() {
                self.occupied = true;
            }
        }
    }
    fn sprite(&self) -> Option<Pt> { if self.occupied { Some (pt(256, 256)) } else { None } }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
    fn speed(&self) -> i32 { 0}
}


pub struct Frog {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    lives: i32,
    blinking: i32,
    count_steps: i32,
    dragging: i32,
    on_raft: bool,
    on_water: bool,
    count_winbox: i32,
    winbox_list: [bool; 5],
    on_free_winbox: bool,
    on_occupied_winbox: bool,
    direction: String, 
}
impl Frog {
    pub fn new(pos: Pt) -> Frog {
        Frog{pos: pos, step: pt(0, 0), size: pt(32, 32),
            speed: 32, lives: 5, blinking: 0, count_steps: 0, dragging: 0, on_raft: false, on_water: false, count_winbox: 0, winbox_list: [false;5], on_free_winbox:false, on_occupied_winbox:false, direction: "Up".to_string()}
    }
    fn lives(&self) -> i32 { self.lives }
    fn get_winbox_list(&self) -> [bool;5] { self.winbox_list }
}
impl Actor for Frog {

    fn act(&mut self, arena: &mut ArenaStatus) {

            self.on_raft = false;
            self.on_water = false;
            self.on_free_winbox = false;
            self.on_occupied_winbox = false;
            //log("entering collision check on_raft false");
            for other  in arena.collisions() {
                if let Some(winbox) = other.as_any().downcast_ref::<WinBox>() {
                    for i in 0..5 {
                        if winbox.get_number() == i as i32 && self.winbox_list[i] == false {
                            self.on_free_winbox = true; 
                            self.winbox_list[i] = true;                    
                        }
                        else {
                            self.on_occupied_winbox = true;
                        }
                    }
                    
                    // if  winbox.get_occupied() {
                    //     self.on_occupied_winbox = true;
                    //     log("on occupied winbox");
                    //     //let mut winbox_mut = winbox.as_mut();
                    //     self.modify_occupied_winbox(winbox, true);
                    // }
                    // else {
                    //     *winbox.occupied() = true;
                    //     self.on_free_winbox = true;
                    //     log("on free winbox");
                    // }
                    
                }
                if let Some(_) = other.as_any().downcast_ref::<Vehicle>() {
                    self.blinking = 20;
                    self.lives -= 1;
                    self.pos = pt(arena.size().x/2, arena.size().y - 32);
                    self.direction = "Up".to_string();
                }
                if let Some(_) = other.as_any().downcast_ref::<Raft>() {
                    self.on_raft = true;
                    log("on_raft true");
                    if self.count_steps == 0 {
                        self.dragging = other.speed();
                    }
                }
                if let Some(_) = other.as_any().downcast_ref::<Water>() {
                    //self.lives -= 1;
                    self.on_water = true;
                    log("on water true");
                }
                if let Some(_) = other.as_any().downcast_ref::<Turtle>() {
                    if other.sprite().is_some() {
                        self.on_raft = true;
                    }
                    
                    if self.count_steps == 0 {
                        self.dragging = other.speed();
                    }
                }
                

            
        }

        if (!self.on_raft || self.on_occupied_winbox) && self.on_water && !self.on_free_winbox {
            self.blinking = 20;
            self.lives -= 1;
            self.pos = pt(arena.size().x/2, arena.size().y - 32);
            self.direction = "Up".to_string();
        }
        else if self.on_free_winbox && self.on_water && !self.on_raft {
            self.count_winbox += 1;
            self.pos = pt(arena.size().x/2, arena.size().y - 32);
            log("countwin incremented")
        }

        let keys = arena.current_keys();
        self.step = pt(0, 0);

        if self.count_steps == 0 {

            if keys.contains(&"ArrowUp") {
                self.count_steps = self.speed;
                self.step.y = -self.speed;
                self.step.x = 0;
                self.direction = "Up".to_string();
            } 
            if keys.contains(&"ArrowDown") {
                self.count_steps = self.speed;
                self.step.y = self.speed;
                self.step.x = 0;
                self.direction = "Down".to_string();
            }
            if keys.contains(&"ArrowLeft") {
                self.count_steps = self.speed;
                self.step.x = -self.speed;
                self.step.y = 0;
                self.direction = "Left".to_string();
            } 
            if keys.contains(&"ArrowRight") {
                self.count_steps = self.speed;
                self.step.x = self.speed;
                self.step.y = 0;
                self.direction = "Right".to_string();
            }
            
        }

        if self.count_steps > 0 {
            self.pos.x += self.step.x;
            self.pos.y += self.step.y;
            self.count_steps -= 16;
        }
        self.pos.x += self.dragging;
        self.dragging = 0;
        //self.pos = self.pos + self.step;

        // if (self.pos.y > 64 && self.pos.y < 224) || (self.pos.y > 64 && self.pos.y < 215) && (self.pos.x < self.size.x || self.pos.x > 640 + self.size.x) {
        //     if !self.on_raft {
        //         log("in water and on_raft = false");
        //         self.lives -= 1;
        //     }
        // }

        let scr = arena.size() - self.size;
        if ! self.on_water {
            self.pos.x = min(max(self.pos.x, 0), scr.x);  // clamp
            self.pos.y = min(max(self.pos.y, 0), scr.y);  // clamp
        }

        if self.pos.x > 640 || self.pos.x < -32 {
            self.blinking = 20;
            self.lives -= 1;
            self.pos = pt(arena.size().x/2, arena.size().y - 32);
            self.direction = "Up".to_string();
        }
        
        self.blinking = max(self.blinking - 1, 0);

    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> {
        if self.blinking > 0 && (self.blinking / 2) % 2 == 0 { None } 
        else {
            if self.direction == "Up" {
                Some(pt(0, 0))   
            }
            else if self.direction == "Down" {
                Some(pt(160, 32))
            }
            else if self.direction == "Left" {
                Some(pt(96, 0))
            }
            else if self.direction == "Right" {
                Some(pt(64, 32))
            }
            else {
                None
            }
        }
    }
    fn alive(&self) -> bool { self.lives > 0 }
    fn as_any(&self) -> &dyn Any { self }
    fn speed(&self) -> i32 { self.speed }
}


pub struct BounceGame {
    arena: Arena,
}
impl BounceGame {
    // fn randpt(size: Pt) -> Pt {
    //     let mut p = pt(randint(0, size.x), randint(0, size.y));
    //     while (p.x - size.x / 2).pow(2) + (p.y - size.y / 2).pow(2) < 10000 {
    //         p = pt(randint(0, size.x), randint(0, size.y));
    //     }
    //     return p;
    // }
    pub fn new(size: Pt, nvehicles: i32, nrafts: i32, nturtles: i32) -> BounceGame {
        let mut arena = Arena::new(size);
        //let size = size - pt(20, 20);
        arena.spawn(Box::new(Water::new(pt(-32,32))));
        for i in 0..5
        {
            arena.spawn(Box::new(WinBox::new(pt(48 + i*128,32), i)));
        }

        // for i in 0..5 {
        //     arena.spawn(Box::new(WinBox::new(pt(32,32))));
        // }

        for i in 0..5 {
            let mut updatepos = 0;
            let mut speed = randint(1, 5);
            
            if  i%2 != 0 {
                speed = - speed
            }
            for _ in 0..nvehicles {
                //arena.spawn(Box::new(Vehicle::new(BounceGame::randpt(size), true, -1)));
                let car = randint(0, 1);
                arena.spawn(Box::new(Vehicle::new(pt(updatepos, 384-(32*i)), if car == 1 {true} else {false}, speed)));
                updatepos += randint(70, 250);
            }
        }

        // for i in 0..5 {
        //     let mut updatepos = 0;
        //     let speed = randint(1, 4);
            
        //     for _ in 0..ntrunks {
        //         //arena.spawn(Box::new(Vehicle::new(BounceGame::randpt(size), true, -1)));
        //         //let car = randint(0, 1);
        //         arena.spawn(Box::new(Raft::new(pt(updatepos, 192-(32*i)), speed, randint(0, 1))));
        //         updatepos += randint(100, 350);
        //     }
        // }

        for i in 0..5 {
            let mut updatepos = 0;
            let speed = randint(1, 3);

            // for _ in 0..nrafts {
            //     arena.spawn(Box::new(Raft::new(pt(updatepos, 192-(32*i)), speed, randint(0, 1))));
            //     updatepos += randint(100, 350);
            // }

            if i%2 == 0 {
                for _ in 0..nturtles {
                //arena.spawn(Box::new(Vehicle::new(BounceGame::randpt(size), true, -1)));
                //let car = randint(0, 1);
                    let sprite_tick = randint(1, 10);
                    let under_water = randint(0,1);
                    for n in 0..randint(2, 3)
                    {
                        arena.spawn(Box::new(Turtle::new(pt(updatepos + 32*n, 192-(32*i)), -speed, under_water, sprite_tick)));
                    }
                    updatepos += randint(100, 250); 
                }
            }
            else {
                for _ in 0..nrafts {
                    arena.spawn(Box::new(Raft::new(pt(updatepos, 192-(32*i)), speed, randint(0, 1))));
                    updatepos += randint(100, 250);
                }
            }

            arena.spawn(Box::new(Frog::new(pt(arena.size().x/2, arena.size().y - 32))));

        }

        // for _ in 0..nghosts {
        //     arena.spawn(Box::new(Ghost::new(BounceGame::randpt(size))));
        // }
        BounceGame{arena: arena}
    }
    pub fn game_over(&self) -> bool { self.remaining_lives() <= 0 }
    pub fn game_won(&self) -> bool { self.winbox_occupied().iter().all(|&value| value == true) }
    pub fn winbox_occupied (&self) -> [bool; 5] {
        let mut winbox = [false; 5];
        let actors = self.actors();
        for a in actors
        {
            if let Some(hero) = a.as_any().downcast_ref::<Frog>() {
                winbox = hero.get_winbox_list();
                return winbox;
            }
        }
        winbox
    }
    pub fn remaining_lives(&self) -> i32 {
        let mut lives = 0;
        let actors = self.actors();
        for a in actors
        {
            if let Some(hero) = a.as_any().downcast_ref::<Frog>() {
                lives = hero.lives();
                return lives;
            }
        }
        lives
    }
    pub fn tick(&mut self, keys: String) { self.arena.tick(keys); }
    pub fn size(&self) -> Pt { self.arena.size() }
    pub fn actors(&self) -> &Vec<Box<dyn Actor>> { self.arena.actors() }
}
