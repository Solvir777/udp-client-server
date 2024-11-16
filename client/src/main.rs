mod client;

use std::thread;
use std::time::Duration;
use flo_canvas::{Color, GraphicsContext, GraphicsPrimitives, LayerId};
use flo_draw::{create_drawing_window, with_2d_graphics};
use shared::sleep;
use crate::client::Client;

fn main() {
    
    with_2d_graphics(|| {
        // Create a window
        let canvas = create_drawing_window("Circular Motion");
        
        // Animation parameters
        let radius = 1.0;
        let center_x = 0.0;
        let center_y = 0.0;
        let speed = 0.05; // Speed of the animation
        
        for t in 0..628 {
            // Calculate the new position of the circle
            let angle = t as f64 * speed;
            let x = center_x + radius * (angle.cos()) as f32;
            let y = center_y + radius * (angle.sin()) as f32;
            
            // Draw the circle at the new position
            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(1.0, 1.0, 1.0, 1.0));
                gc.new_path();
                gc.circle(x, y, radius);
                gc.fill_color(Color::Rgba(0.0, 0.0, 0.5, 1.0));
                gc.fill();
            });
            sleep(16);
        }
    });
    
    
    let mut client = Client::new(8080..9000).expect("Couldn't create client!");
    client.run();
}