use wasm_bindgen::prelude::*;
use std::cell::RefCell;

pub mod actor;
pub mod bounce;
pub mod g2d;
pub mod pt2d;
pub mod rand;

pub struct BounceGui {
    game: bounce::BounceGame
}
impl BounceGui {
    pub fn new() -> BounceGui {
        let game = bounce::BounceGame::new(pt2d::pt(640, 448), 2, 5, 5);
        BounceGui{game}
    }
    pub fn setup(&self) {
        g2d::init_canvas(self.game.size());
        g2d::main_loop(30);
    }
    pub fn tick(&mut self) {
        g2d::clear_canvas();
        g2d::draw_image_clip("frogger-bg.png".to_string(), pt2d::pt(0, 0), pt2d::pt(0, 16), pt2d::pt(640, 448));
        for b in self.game.actors() {
            if let Some(img) = b.sprite() {
                g2d::draw_image_clip("frogger.png".to_string(), b.pos(), img, b.size());
            } else {
                //g2d::fill_rect(b.pos(), b.size());
            }
        }
        //let red = g2d::Color(255, 0, 0);
        g2d::set_color(255, 0, 0);
        let txt = format!("Lives: {}",
            self.game.remaining_lives());
        g2d::draw_text(txt, pt2d::pt(0, 0), 20);

        if self.game.game_over() {
            g2d::alert("Game over".to_string());
            g2d::close_canvas();
        } else if self.game.game_won() {
            g2d::alert("Game won".to_string());
            g2d::close_canvas();
        } else {
            self.game.tick(g2d::current_keys());  // Game logic
        }
    }
}

thread_local! {
    static GUI: RefCell<BounceGui> = RefCell::new(BounceGui::new());
}

#[wasm_bindgen]
pub fn tick() {
    GUI.with(|g| {
        g.borrow_mut().tick();
    });
}

#[wasm_bindgen]
pub fn setup() {
    GUI.with(|g| {
        g.borrow_mut().setup();
    });
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
