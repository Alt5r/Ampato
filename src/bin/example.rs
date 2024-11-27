use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut pos:f32 = 0.0;
    loop {
        clear_background(BLACK);
        pos += 0.5;

        
        draw_circle(screen_width()/2.0 + pos, screen_height()/2.0, 15.0, BLUE);
        

        next_frame().await
    }
}