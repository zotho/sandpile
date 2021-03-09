pub struct Field {
    pub height: usize,
    pub width: usize,
    pub inner_field: Vec<bool>,
    pub old_field: Vec<bool>,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        let inner_field = vec![false; width * height];
        let old_field = inner_field.clone();
        Field {
            width, height, inner_field, old_field
        }
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.inner_field, &mut self.old_field);
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.inner_field[self.index(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut bool {
        let index = self.index(x, y);
        &mut self.inner_field[index]
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn update(&mut self) {
        self.swap();
        for y in 0..self.height {
            for x in 0..self.width {
                let start_x = x.saturating_sub(1);
                let start_y = y.saturating_sub(1);
                let end_x = (x + 1).min(self.width - 1);
                let end_y = (y + 1).min(self.height - 1);
                let mut sum = 0;
                for inner_x in start_x..=end_x {
                    for inner_y in start_y..=end_y {
                        if self.old_field[self.index(inner_x, inner_y)] && !(x == inner_x && y == inner_y)
                        {
                            sum += 1;
                        }
                    }
                }
                let current_index = self.index(x, y);
                let current_cell = self.old_field[current_index];

                // N3, S23
                self.inner_field[current_index] = matches!((sum, current_cell),(2, true) | (3, _));
            }
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize) {
        *self.get_mut(x, y) = true;
    }

    pub fn put_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        // Calculate line deltas
        let dx = x2 as i16 - x1 as i16;
        let dy = y2 as i16 - y1 as i16;
        // Create a positive copy of deltas (makes iterating easier)
        let dx1 = dx.abs();
        let dy1 = dy.abs();
        // Calculate error intervals for both axis
        let mut px = 2 * dy1 - dx1;
        let mut py = 2 * dx1 - dy1;
        // The line is X-axis dominant
        if dy1 <= dx1 {
            // Line is drawn left to right
            let (x, mut y, xe) = if dx >= 0 {
                (x1, y1, x2)
            } else { // Line is drawn right to left (swap ends)
                (x2, y2, x1)
            };
            self.put_pixel(x, y); // Draw first pixel
            // Rasterize the line
            for x in x..xe {
                // Deal with octants...
                if px < 0 {
                    px = px + 2 * dy1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                        y = y + 1;
                    } else {
                        y = y - 1;
                    }
                    px = px + 2 * (dy1 - dx1);
                }
                // Draw pixel from line span at
                // currently rasterized position
                self.put_pixel(x, y);
            }
        } else { // The line is Y-axis dominant
            // Line is drawn bottom to top
            let (mut x, y, ye) = if dy >= 0 {
                (x1, y1, y2)
            } else { // Line is drawn top to bottom
                (x2, y2, y1)
            };
            self.put_pixel(x, y); // Draw first pixel
            // Rasterize the line
            for y in y..ye {
                // Deal with octants...
                if py <= 0 {
                    py = py + 2 * dx1;
                } else {
                    if (dx < 0 && dy<0) || (dx > 0 && dy > 0) {
                        x = x + 1;
                    } else {
                        x = x - 1;
                    }
                    py = py + 2 * (dx1 - dy1);
                }
                // Draw pixel from line span at
                // currently rasterized position
                self.put_pixel(x, y);
            }
        }
    }
}