use serde::{Deserialize, Serialize};

use crate::add_assign_signed::AddAssignSigned;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Field {
    pub height: usize,
    pub width: usize,
    pub inner_field: Vec<u32>,
    pub old_field: Vec<u32>,
    pub job_queue: Vec<(usize, usize, usize)>,
    pub old_job_queue: Vec<(usize, usize, usize)>,
    pub iteration: usize,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        let inner_field = vec![0; width * height];
        let old_field = inner_field.clone();
        Field {
            width,
            height,
            inner_field,
            old_field,
            job_queue: Vec::new(),
            old_job_queue: Vec::new(),
            iteration: 0,
        }
    }

    pub fn _fill_job_queue(&mut self) {
        let mut index = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let previous_count = self.inner_field[index];

                if previous_count > 3 {
                    self.job_queue.push((index, x, y));
                }

                index += 1;
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> u32 {
        self.inner_field[self.index(x, y)]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut u32 {
        let index = self.index(x, y);
        self.job_queue.push((index, x, y));
        &mut self.inner_field[index]
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn check_coords(&self, x: f32, y: f32) -> (usize, usize) {
        let (x, y) = if x >= 0.0 && y >= 0.0 {
            (x as usize, y as usize)
        } else {
            (x.max(0.0) as usize, y.max(0.0) as usize)
        };
        (x.min(self.width - 1), y.min(self.height - 1))
    }

    pub fn update(&mut self) {
        // let t1 = std::time::Instant::now();
        std::mem::swap(&mut self.inner_field, &mut self.old_field);
        std::mem::swap(&mut self.job_queue, &mut self.old_job_queue);

        // let t2 = std::time::Instant::now();
        self.inner_field.clone_from(&self.old_field);
        // let t2e = t2.elapsed().as_secs_f64();

        // let mut prev_y: usize = self.old_job_queue.first().unwrap().2;
        // let chunks = self.old_job_queue.split_mut(|(_, _, y)| {
        //     println!("{} {}", prev_y, y);
        //     if *y != prev_y {
        //         prev_y = *y;
        //         true
        //     } else {
        //         false
        //     }
        // });
        // for group in chunks {
        //     println!("{:?}", group.len());
        // }
        for i in 0..self.old_job_queue.len() {
            let (index, x, y) = self.old_job_queue[i];
            let previous_count = self.old_field[index];

            if previous_count > 3 {
                self.add_to(index, x, y, -4);

                if x > 0 {
                    self.add_to(index - 1, x - 1, y, 1);
                }
                if x < self.width - 1 {
                    self.add_to(index + 1, x + 1, y, 1);
                }
                if y > 0 {
                    self.add_to(index - self.width, x, y - 1, 1);
                }
                if y < self.height - 1 {
                    self.add_to(index + self.width, x, y + 1, 1);
                }
            }
        }
        self.old_job_queue.clear();

        self.job_queue.sort_unstable_by_key(|job| job.0);
        self.job_queue.dedup();
        if self.job_queue.len() != 0 {
            self.iteration += 1;
        }

        // let t1e = t1.elapsed().as_secs_f64();
        // println!("{}/{} == {}", t2e, t1e, t2e / t1e);
    }

    pub fn add_to(&mut self, index: usize, x: usize, y: usize, amount: i64) {
        self.inner_field[index].add_assign_signed(amount);
        if self.inner_field[index] > 3 {
            self.job_queue.push((index, x, y));
        }
    }

    pub fn _slow_update(&mut self) {
        std::mem::swap(&mut self.inner_field, &mut self.old_field);

        self.inner_field.clone_from(&self.old_field);

        let mut index = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let previous_count = self.old_field[index];

                if previous_count > 3 {
                    self.inner_field[index] -= 4;
                    if x > 0 {
                        self.inner_field[index - 1] += 1;
                    }
                    if x < self.width - 1 {
                        self.inner_field[index + 1] += 1;
                    }
                    if y > 0 {
                        self.inner_field[index - self.width] += 1;
                    }
                    if y < self.height - 1 {
                        self.inner_field[index + self.width] += 1;
                    }
                }

                index += 1;
            }
        }
    }

    pub fn put_pixel(&mut self, x: usize, y: usize) {
        let index = self.index(x, y);
        self.add_to(index, x, y, 40);
    }

    pub fn put_line(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        // Source: https://jstutorial.medium.com/how-to-code-your-first-algorithm-draw-a-line-ca121f9a1395
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
            } else {
                // Line is drawn right to left (swap ends)
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
        } else {
            // The line is Y-axis dominant
            // Line is drawn bottom to top
            let (mut x, y, ye) = if dy >= 0 {
                (x1, y1, y2)
            } else {
                // Line is drawn top to bottom
                (x2, y2, y1)
            };
            self.put_pixel(x, y); // Draw first pixel
                                  // Rasterize the line
            for y in y..ye {
                // Deal with octants...
                if py <= 0 {
                    py = py + 2 * dx1;
                } else {
                    if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
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

        self.job_queue.sort_unstable();
        self.job_queue.dedup();
    }
}
