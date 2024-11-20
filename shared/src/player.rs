pub struct Player {
	pub x: f32,
	pub y: f32,
	pub size: f32,
}

impl Player {
	const SPEED: f32 = 1.0;
	fn new() -> Self {
		Self{
			x: 0.,
			y: 0.,
			size: 5.,
		}
	}
}