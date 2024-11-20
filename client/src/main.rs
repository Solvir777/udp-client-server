mod client;
mod canvas;

use flo_canvas::{Color, GraphicsContext, GraphicsPrimitives};
use flo_draw::{create_drawing_window, with_2d_graphics};
use crate::client::Client;

fn main() {
    with_2d_graphics(|| {
        let mut client = Client::new(8080..9000).expect("Couldn't create client!");
        client.run();
        let mut canvas = create_drawing_window("Client");
        // Animation parameters
        let radius = 1.0;
        let center_x = 0.0;
        let center_y = 0.0;
        let speed = 0.05; // Speed of the animation
        let mut a = 0;
        loop {
            a+=1;
            // Calculate the new position of the circle
            let angle = a as f64 * speed;
            let x = center_x + radius * angle.cos() as f32;
            let y = center_y + radius * angle.sin() as f32;
            
            // Draw the circle at the new position
            canvas.draw(|gc| {
                gc.clear_canvas(Color::Rgba(0.0, 0.0, 0.0, 1.0));
                gc.new_path();
                gc.circle(x, y, radius);
                gc.fill_color(Color::Rgba(0.0, 0.0, 0.5, 1.0));
                gc.fill();
            });
        }
    });
}