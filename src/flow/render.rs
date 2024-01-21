use crate::*;

impl GuiS {
	pub fn render(&self, buffer: &mut BufferS) {
		for window in &self.window {
			if window.draw {
				WindowS::draw(window, buffer);
			}
		}

		for button in &self.button {
			if button.draw {

			}
		}

		for table in &self.table {
			if table.draw {

			}
		}
	}

	pub fn draw_block(
		aabb: &AabbT, 
		line_width: usize, 
		color_main: u32,
		color_border: Option<u32>,
		buffer: &mut BufferS
	) {
		if let Some(color) = color_border {
			BufferS::draw_border(aabb, line_width, color, buffer);

			let aabb = [
				[aabb[0][0] + line_width, aabb[0][1] + line_width],
				[aabb[1][0] - line_width, aabb[1][1] - line_width]
			];

			BufferS::draw_square(&aabb, color_main, buffer);
		} else {
			BufferS::draw_square(&aabb, color_main, buffer);
		}
	}
}

impl WindowS {
	pub fn draw(&self, buffer: &mut BufferS) {
		GuiS::draw_block(
			&self.aabb, 
			self.line_width, 
			self.color_main, 
			self.color_border, 
			buffer
		);

		for button in &self.button {
			if button.draw {
				ButtonS::draw(&button, buffer);
			}
		}
	}
}

impl ButtonS {
	pub fn draw(&self, buffer: &mut BufferS) {
		GuiS::draw_block(
			&self.aabb, 
			self.line_width, 
			self.color_main, 
			self.color_border, 
			buffer
		);
	}
}

//

impl BufferS {
	pub fn draw_line(
		posit: &AabbT, 
		line_width: usize, 
		color: u32,
		buffer: &mut BufferS
	) {
	    let dx = (posit[1][0] as isize - posit[0][0] as isize).abs();
	    let dy = (posit[1][1] as isize - posit[0][1] as isize).abs();
	    let sx = if posit[0][0] < posit[1][0] { 1 } else { -1 };
	    let sy = if posit[0][1] < posit[1][1] { 1 } else { -1 };
	    let mut err = dx - dy;

	    let mut x = posit[0][0] as isize;
	    let mut y = posit[0][1] as isize;

	    while x != posit[1][0] as isize || y != posit[1][1] as isize {
	        for i in 0..line_width {
	            for j in 0..line_width {
	                let nx = x + i as isize - line_width as isize / 2;
	                let ny = y + j as isize - line_width as isize / 2;

	                if nx >= 0
	                    && nx < buffer.size[0] as isize
	                    && ny >= 0
	                    && ny < buffer.size[1] as isize
	                {
	                    let index = (ny * buffer.size[0] as isize + nx) as usize;
	                    buffer.buffer[index] = color; // Красный цвет
	                }
	            }
	        }

	        let e2 = 2 * err;
	        if e2 > -dy {
	            err -= dy;
	            x += sx;
	        }
	        if e2 < dx {
	            err += dx;
	            y += sy;
	        }
	    }
	}

	pub fn draw_border(
		aabb: &AabbT, 
		line_width: usize, 
		color: u32,
		buffer: &mut BufferS
	) {
	    let lw = line_width / 2;

	    let q_0 = [aabb[0][0] + lw, aabb[0][1] + lw];
	    let q_1 = [aabb[0][0] + aabb[1][0] - lw, aabb[0][1] + lw];
	    let q_2 = [aabb[0][0] + aabb[1][0] - lw, aabb[0][1] + aabb[1][1] - lw];
	    let q_3 = [aabb[0][0] + lw, aabb[0][1] + aabb[1][1] - lw];

	    Self::draw_line(&[q_0, q_1], line_width, color, buffer);

	    Self::draw_line(&[q_1, q_2], line_width, color, buffer);

	    Self::draw_line(&[q_2, q_3], line_width, color, buffer);

	    Self::draw_line(&[q_3, q_0], line_width, color, buffer);
	}

	pub fn draw_square(aabb: &AabbT, color: u32, buffer: &mut BufferS) {
		for y in aabb[0][1]..=(aabb[0][1] + aabb[1][1]) {
			for x in aabb[0][0]..=(aabb[0][0] + aabb[1][0]) {
				let index = y * buffer.size[0] + x;

				buffer.buffer[index] = color;
			}
		}
	}
}