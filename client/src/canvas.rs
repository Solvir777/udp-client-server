use flo_canvas::{Color, DrawingTarget, GraphicsContext, GraphicsPrimitives};
use flo_draw::create_drawing_window;
use shared::player::Player;

pub struct Canvas{
	drawing_target: DrawingTarget,
}


impl Canvas{
	pub fn new(name: &str) -> Self{
		Canvas{drawing_target: create_drawing_window(name)}
	}
	
	
	pub fn draw(&mut self, objects: &Vec<Player>) {
		self.drawing_target.draw(|gc| {
			gc.clear_canvas(Color::Rgba(0., 0., 0., 0.));
			
			for player in objects{
				gc.new_path();
				gc.circle(player.x, player.y, player.size);
				gc.fill();
			}
			
		});
	}
}