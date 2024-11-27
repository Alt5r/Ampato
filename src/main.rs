use comfy::*;

// Declare a static mutable variable to hold the position of the circle
static mut XP: f32 = 0.0;

simple_game!("Nice red circle", update);

fn update(_c: &mut EngineContext) {
    // Update the position of the circle
    unsafe {
        XP += 0.1; // Increment the position
        if XP > 4.0 {  // If the ball goes off-screen, reset its position
            XP = -1.0;
        }
    }

    // Draw the circle at the updated position
    draw_circle(vec2(unsafe { XP }, 0.0), 0.5, RED, 0);
    draw_circle(vec2(0.0, unsafe { XP }), 0.5, BLUE, 1);
}