use std::any::Any;
use std::cmp::{min, max};

use crate::{actor::*, log};
use crate::rand::*;

pub struct Trunk {
    pos: Pt,
    step: Pt,
    size: Pt,
    speed: i32,
    type_trunk: i32,
}
impl Trunk {
    pub fn new(pos: Pt, speed: i32, type_trunk: i32) -> Trunk {
        Trunk{pos: pos, step: pt(1, 0), size: pt(if type_trunk == 0 { 64 } else { 96 } , 32), speed: speed, type_trunk: type_trunk}
    }
}
impl Actor for Trunk {
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


pub struct Ghost {
    pos: Pt,
    speed: i32,
    visible: bool
}
impl Ghost {
    pub fn new(pos: Pt) -> Ghost {
        Ghost{pos: pos, speed: 4, visible: true}
    }
}
impl Actor for Ghost {
    fn act(&mut self, arena: &mut ArenaStatus) {
        let scr = arena.size();
        let step = pt(randint(-1, 1) * self.speed, randint(-1, 1) * self.speed);
        self.pos = self.pos + step + scr;
        self.pos.x %= scr.x;
        self.pos.y %= scr.y;
        if randint(0, 99) == 0 { self.visible = ! self.visible; }
        //if randint(0, 999) == 0 { arena.spawn(Box::new(Vehicle::new(self.pos))); }
    }
    fn sprite(&self) -> Option<Pt> { 
        Some(pt(20, if self.visible { 0 } else { 20 })) 
    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { pt(20, 20) }
    fn alive(&self) -> bool { true }
    fn as_any(&self) -> &dyn Any { self }
    fn speed(&self) -> i32 { self.speed }
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
}
impl Frog {
    pub fn new(pos: Pt) -> Frog {
        Frog{pos: pos, step: pt(0, 0), size: pt(32, 32),
            speed: 32, lives: 3, blinking: 0, count_steps: 0, dragging: 0, on_raft: false}
    }
    fn lives(&self) -> i32 { self.lives }
}
impl Actor for Frog {
    fn act(&mut self, arena: &mut ArenaStatus) {
        if self.blinking == 0 {
            self.on_raft = false;
            for other in arena.collisions() {
                if let Some(_) = other.as_any().downcast_ref::<Vehicle>() {
                    self.blinking = 60;
                    self.lives -= 1;
                }
                if let Some(_) = other.as_any().downcast_ref::<Trunk>() {
                    self.on_raft = true;
                    if self.count_steps == 0 {
                        self.dragging = other.speed();
                    }
                }
            }
        }
        let keys = arena.current_keys();
        self.step = pt(0, 0);

        if self.count_steps == 0 {

            if keys.contains(&"ArrowUp") {
                self.count_steps = self.speed;
                self.step.y = -self.speed;
                self.step.x = 0;
            } 
            if keys.contains(&"ArrowDown") {
                self.count_steps = self.speed;
                self.step.y = self.speed;
                self.step.x = 0;
            }
            if keys.contains(&"ArrowLeft") {
                self.count_steps = self.speed;
                self.step.x = -self.speed;
                self.step.y = 0;
            } 
            if keys.contains(&"ArrowRight") {
                self.count_steps = self.speed;
                self.step.x = self.speed;
                self.step.y = 0;
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

        if (self.pos.y > 64 && self.pos.y < 224) || (self.pos.y > 64 && self.pos.y < 224) && (self.pos.x < self.size.x || self.pos.x > 640 + self.size.x) {
            if !self.on_raft {
                self.lives -= 1;
            }
        }

        let scr = arena.size() - self.size;
        self.pos.x = min(max(self.pos.x, 0), scr.x);  // clamp
        self.pos.y = min(max(self.pos.y, 0), scr.y);  // clamp
        self.blinking = max(self.blinking - 1, 0);

    }
    fn pos(&self) -> Pt { self.pos }
    fn size(&self) -> Pt { self.size }
    fn sprite(&self) -> Option<Pt> {
        if self.blinking > 0 && (self.blinking / 2) % 2 == 0 { None }
        else { Some(pt(0, 0)) }
    }
    fn alive(&self) -> bool { self.lives > 0 }
    fn as_any(&self) -> &dyn Any { self }
    fn speed(&self) -> i32 { self.speed }
}


pub struct BounceGame {
    arena: Arena,
    playtime: i32
}
impl BounceGame {
    fn randpt(size: Pt) -> Pt {
        let mut p = pt(randint(0, size.x), randint(0, size.y));
        while (p.x - size.x / 2).pow(2) + (p.y - size.y / 2).pow(2) < 10000 {
            p = pt(randint(0, size.x), randint(0, size.y));
        }
        return p;
    }
    pub fn new(size: Pt, nvehicles: i32, ntrunks: i32) -> BounceGame {
        let mut arena = Arena::new(size);
        //let size = size - pt(20, 20);
        arena.spawn(Box::new(Frog::new(pt(arena.size().x/2, arena.size().y - 32))));
        for i in 0..5 {
            let mut updatepos = 0;
            let mut speed = randint(2, 7);
            
            if  i%2 != 0 {
                speed = - speed
            }
            for _ in 0..nvehicles {
                //arena.spawn(Box::new(Vehicle::new(BounceGame::randpt(size), true, -1)));
                let car = randint(0, 1);
                arena.spawn(Box::new(Vehicle::new(pt(updatepos, 384-(32*i)), if car == 1 {true} else {false}, speed)));
                updatepos += randint(70, 300);
            }
        }

        for i in 0..ntrunks {
            let mut updatepos = 0;
            let speed = randint(1, 4);
            
            for _ in 0..4 {
                //arena.spawn(Box::new(Vehicle::new(BounceGame::randpt(size), true, -1)));
                //let car = randint(0, 1);
                arena.spawn(Box::new(Trunk::new(pt(updatepos, 192-(32*i)), speed, randint(0, 1))));
                updatepos += randint(100, 350);
            }
        }

        arena.spawn(Box::new(Trunk::new(pt(0, 0), 3, 0)));
        arena.spawn(Box::new(Trunk::new(pt(150, 0), 3, 1)));

        // for _ in 0..nghosts {
        //     arena.spawn(Box::new(Ghost::new(BounceGame::randpt(size))));
        // }
        BounceGame{arena: arena, playtime: 120}
    }
    pub fn game_over(&self) -> bool { self.remaining_lives() <= 0 }
    pub fn game_won(&self) -> bool { self.remaining_time() <= 0 }
    pub fn remaining_time(&self) -> i32 {
        self.playtime - self.arena.count() / 30
    }
    pub fn remaining_lives(&self) -> i32 {
        let mut lives = 0;
        let actors = self.actors();
        if let Some(b) = actors.first() {
            if let Some(hero) = b.as_any().downcast_ref::<Frog>() {
                lives = hero.lives();
            }
        }
        lives
    }
    pub fn tick(&mut self, keys: String) { self.arena.tick(keys); }
    pub fn size(&self) -> Pt { self.arena.size() }
    pub fn actors(&self) -> &Vec<Box<dyn Actor>> { self.arena.actors() }
}
