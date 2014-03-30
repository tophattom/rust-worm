#![no_main]

extern crate native;
extern crate rand;
extern crate sdl;

use std::vec;
use rand::{task_rng, Rng, TaskRng};

enum Direction {
    Up = -1,
    Down = 1,
    Left = -2,
    Right = 2
}

#[deriving(Clone)]
struct Piece {
    x: int,
    y: int
}

struct Worm {
    pieces: ~[Piece],
    dir: Direction,
    grow: bool
}

struct Apple {
	x: int,
	y: int
}


impl Worm {
	fn move(&mut self) {
		let head = self.pieces[0].clone();

		match self.dir {
			Up => { self.pieces.unshift(Piece {x: head.x, y: head.y - 1}) }
			Down => { self.pieces.unshift(Piece {x: head.x, y: head.y + 1}) }
			Left => { self.pieces.unshift(Piece {x: head.x - 1, y: head.y}) }
			Right => { self.pieces.unshift(Piece {x: head.x + 1, y: head.y}) }
		}

		if !self.grow {
			self.pieces.pop();
		}

		self.grow = false;
	}

	fn checkCollisions(&self, apple : Apple) -> Result<bool, bool> {
		if self.pieces.len() == 1 {
			return Err(false);
		}

		let head = self.pieces[0].clone();

		for i in range(1, self.pieces.len()) {
			let piece = self.pieces[i].clone();

			if piece.x == head.x && piece.y == head.y {
				return Err(true);
			}
		}

		if head.x == apple.x && head.y == apple.y {
			return Ok(true);
		}

		Err(false)
	}

	fn setDirection(&mut self, newDir : Direction) {
		if self.dir as int != newDir as int * -1 {
			self.dir = newDir;
		}
	}

	fn draw(&self, screen : &sdl::video::Surface, size : int) {
		for i in range(0, self.pieces.len()) {
			let piece = self.pieces[i];
			let mut color = sdl::video::Color::from_struct(&sdl::video::ll::SDL_Color {r: 200, g: 100, b: 100, unused: 0});

			if i == 0 {
				color = sdl::video::Color::from_struct(&sdl::video::ll::SDL_Color {r: 100, g: 200, b: 100, unused: 0});
			}

			screen.fill_rect(Some(sdl::Rect {
				x: (piece.x * size) as i16,
				y: (piece.y * size) as i16,
				w: (size - 1) as u16,
				h: (size - 1) as u16
			}), color);
		}
	}
}


impl Apple {
	fn draw(&self, screen : &sdl::video::Surface, size : int) {
		let color = sdl::video::Color::from_struct(&sdl::video::ll::SDL_Color {r: 200, g: 0, b: 0, unused: 0});

		screen.fill_rect(Some(sdl::Rect {
			x: (self.x * size) as i16,
			y: (self.y * size) as i16,
			w: (size - 1) as u16,
			h: (size - 1) as u16
		}), color);
	}

	fn reposition(&mut self, mut rng : TaskRng, max_x : int, max_y : int) {
		self.x = rng.gen_range(0, max_x);
		self.y = rng.gen_range(0, max_y);
	}
}



#[no_mangle]
pub extern "C" fn SDL_main(argc: int, argv: **u8) {
    native::start(argc, argv, real_main);
}



fn real_main() {
	let screen_width = 400;
	let screen_height = 300;

	sdl::init([sdl::InitVideo]);
	sdl::wm::set_caption("Rusty snake", "rust-worm");

	let screen = match sdl::video::set_video_mode(screen_width,
												  screen_height, 
												  32, 
												  [sdl::video::HWSurface],
                                                  [sdl::video::DoubleBuf]) {
        Ok(screen) => screen,
        Err(err) => fail!("failed to set video mode: {}", err)
    };

    let mut rng = task_rng();
    let background_color = sdl::video::Color::from_struct(&sdl::video::ll::SDL_Color {r: 0, g: 0, b: 0, unused: 0});

    let size = 20;
    let mut delay = 200;
	let mut worm = Worm {pieces: ~[Piece {x: 2, y: 2}, Piece {x: 1, y: 2}, Piece {x: 0, y: 2}], dir: Right, grow: false};

	let max_x : int = (screen_width / size) as int;
	let max_y : int = (screen_height / size) as int;

	let mut apple = Apple {x: rng.gen_range(0, max_x), y: rng.gen_range(0, max_y)};

	'main: loop {
		'event: loop {
			match sdl::event::poll_event() {
				sdl::event::QuitEvent => break 'main,
				sdl::event::NoEvent => break 'event,
				sdl::event::KeyEvent(key, _, _, _)
					=> match key {
						sdl::event::EscapeKey => break 'main,
						sdl::event::UpKey => worm.setDirection(Up),
						sdl::event::DownKey => worm.setDirection(Down),
						sdl::event::LeftKey => worm.setDirection(Left),
						sdl::event::RightKey => worm.setDirection(Right),
						_ => {}
					},
				_ => {}
			}
		}

		//Clear screen
		screen.fill_rect(Some(sdl::Rect {
			x: 0i16,
			y: 0i16,
			w: screen_width as u16,
			h: screen_height as u16,
		}), background_color);

		worm.move();

		apple.draw(screen, size);
		worm.draw(screen, size);

		screen.flip();

		match worm.checkCollisions(apple) {
			Ok(status) if status => {
				worm.grow = true;
				apple.reposition(rng, max_x, max_y);
				delay = std::cmp::max(50, delay - 10);
			}
			Err(status) if status => break 'main,
			_ => {}
		}

		std::io::timer::sleep(delay as u64);
	}
}