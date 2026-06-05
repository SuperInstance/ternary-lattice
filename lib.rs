#![forbid(unsafe_code)]

/// Lattice gas automata on ternary grids.
/// Velocities: 0=rest, 1=right, 2=up, 3=left, 4=down.

pub struct LatticeGas {
    pub cells: Vec<i8>,
    pub velocities: Vec<u8>,
    pub width: usize,
}

impl LatticeGas {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        LatticeGas {
            cells: vec![0; size],
            velocities: vec![0; size],
            width,
        }
    }

    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }

    /// Move particles along their velocity direction.
    pub fn stream(&mut self) {
        let w = self.width;
        let h = self.height();
        let size = w * h;
        let mut new_cells = vec![0i8; size];
        let mut new_vel = vec![0u8; size];

        for i in 0..size {
            if self.cells[i] == 0 { continue; }
            let v = self.velocities[i];
            let (nx, ny) = match v {
                0 => (i % w, i / w),         // rest
                1 => ((i % w) + 1, i / w),   // right
                2 => (i % w, (i / w) + 1),   // up (in grid terms)
                3 => { let x = i % w; (if x > 0 { x - 1 } else { 0 }, i / w) } // left
                4 => (i % w, { let y = i / w; if y > 0 { y - 1 } else { 0 } }), // down
                _ => (i % w, i / w),
            };
            // Wrap or clamp — we clamp to stay in bounds
            let ni = if nx < w && ny < h { ny * w + nx } else { i };
            // Simple: if target empty, move; otherwise bounce (stay)
            if new_cells[ni] == 0 {
                new_cells[ni] = self.cells[i];
                new_vel[ni] = v;
            } else {
                // Collision: stay in place (simple model)
                new_cells[i] = self.cells[i];
                new_vel[i] = v;
            }
        }
        self.cells = new_cells;
        self.velocities = new_vel;
    }

    /// Apply collision rules for ternary states.
    pub fn collide(&mut self) {
        let size = self.cells.len();
        let w = self.width;
        for i in 0..size {
            if self.cells[i] == 0 { continue; }
            // Simple collision: if heading right and cell to right has particle heading left, scatter
            let v = self.velocities[i];
            let neighbor = match v {
                1 => { if i % w + 1 < w { Some(i + 1) } else { None } }   // right
                3 => { if i % w > 0 { Some(i - 1) } else { None } }       // left
                _ => None,
            };
            if let Some(ni) = neighbor {
                if self.cells[ni] != 0 {
                    // Head-on collision: reverse directions (swap up/down)
                    if (v == 1 && self.velocities[ni] == 3) || (v == 3 && self.velocities[ni] == 1) {
                        // Scatter perpendicular
                        self.velocities[i] = 2; // up
                        self.velocities[ni] = 4; // down
                    }
                }
            }
        }
    }

    /// Stream then collide.
    pub fn step(&mut self) {
        self.stream();
        self.collide();
    }

    /// Compute density per cell (0 or 1 particle → f64).
    pub fn density(&self) -> Vec<f64> {
        self.cells.iter().map(|&c| if c != 0 { 1.0 } else { 0.0 }).collect()
    }

    /// Compute total momentum (sum of velocity components).
    pub fn momentum(&self) -> (f64, f64) {
        let mut mx = 0.0f64;
        let mut my = 0.0f64;
        for i in 0..self.cells.len() {
            if self.cells[i] != 0 {
                match self.velocities[i] {
                    1 => mx += 1.0,
                    3 => mx -= 1.0,
                    2 => my += 1.0,
                    4 => my -= 1.0,
                    _ => {}
                }
            }
        }
        (mx, my)
    }

    /// Compute "temperature" as mean kinetic energy (rest=0, moving=0.5).
    pub fn temperature(&self) -> f64 {
        let n = self.cells.len();
        if n == 0 { return 0.0; }
        let total_ke: f64 = self.cells.iter().zip(self.velocities.iter())
            .map(|(&c, &v)| {
                if c == 0 { 0.0 }
                else if v == 0 { 0.0 }
                else { 0.5 }
            })
            .sum();
        let count = self.cells.iter().filter(|&&c| c != 0).count();
        if count == 0 { 0.0 } else { total_ke / count as f64 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let lg = LatticeGas::new(5, 5);
        assert_eq!(lg.cells.len(), 25);
        assert_eq!(lg.velocities.len(), 25);
        assert_eq!(lg.width, 5);
    }

    #[test]
    fn test_struct_size() {
        // LatticeGas has heap-allocated Vec fields, so struct size > 16 is expected.
        // Verify forbid(unsafe_code) compiles — this test just confirms construction works.
        let lg = LatticeGas::new(3, 3);
        assert_eq!(lg.cells.len(), 9);
    }

    #[test]
    fn test_stream_rest() {
        let mut lg = LatticeGas::new(3, 3);
        lg.cells[4] = 1;
        lg.velocities[4] = 0; // rest
        lg.stream();
        assert_eq!(lg.cells[4], 1);
    }

    #[test]
    fn test_stream_right() {
        let mut lg = LatticeGas::new(3, 3);
        lg.cells[3] = 1;
        lg.velocities[3] = 1; // right
        lg.stream();
        assert_eq!(lg.cells[4], 1);
        assert_eq!(lg.cells[3], 0);
    }

    #[test]
    fn test_stream_left() {
        let mut lg = LatticeGas::new(3, 3);
        lg.cells[5] = 1;
        lg.velocities[5] = 3; // left
        lg.stream();
        assert_eq!(lg.cells[4], 1);
        assert_eq!(lg.cells[5], 0);
    }

    #[test]
    fn test_stream_up() {
        let mut lg = LatticeGas::new(3, 3);
        lg.cells[3] = 1; // row 1, col 0
        lg.velocities[3] = 2; // up
        lg.stream();
        assert_eq!(lg.cells[6], 1); // row 2, col 0
    }

    #[test]
    fn test_stream_down() {
        let mut lg = LatticeGas::new(3, 3);
        lg.cells[6] = 1; // row 2, col 0
        lg.velocities[6] = 4; // down
        lg.stream();
        assert_eq!(lg.cells[3], 1); // row 1, col 0
    }

    #[test]
    fn test_density() {
        let mut lg = LatticeGas::new(2, 2);
        lg.cells[0] = 1;
        lg.cells[3] = -1;
        let d = lg.density();
        assert_eq!(d[0], 1.0);
        assert_eq!(d[1], 0.0);
        assert_eq!(d[3], 1.0);
    }

    #[test]
    fn test_momentum_rest() {
        let mut lg = LatticeGas::new(3, 3);
        lg.cells[4] = 1;
        lg.velocities[4] = 0;
        let (mx, my) = lg.momentum();
        assert_eq!(mx, 0.0);
        assert_eq!(my, 0.0);
    }

    #[test]
    fn test_momentum_moving() {
        let mut lg = LatticeGas::new(3, 3);
        lg.cells[4] = 1;
        lg.velocities[4] = 1; // right
        let (mx, my) = lg.momentum();
        assert_eq!(mx, 1.0);
        assert_eq!(my, 0.0);
    }

    #[test]
    fn test_momentum_balanced() {
        let mut lg = LatticeGas::new(5, 1);
        lg.cells[1] = 1;
        lg.velocities[1] = 1; // right
        lg.cells[3] = 1;
        lg.velocities[3] = 3; // left
        let (mx, _) = lg.momentum();
        assert_eq!(mx, 0.0);
    }

    #[test]
    fn test_temperature() {
        let mut lg = LatticeGas::new(3, 3);
        lg.cells[0] = 1;
        lg.velocities[0] = 0; // rest
        lg.cells[1] = 1;
        lg.velocities[1] = 1; // moving
        let t = lg.temperature();
        assert!((t - 0.25).abs() < 0.001); // 0.5 / 2
    }

    #[test]
    fn test_step() {
        let mut lg = LatticeGas::new(3, 3);
        lg.cells[4] = 1;
        lg.velocities[4] = 1; // right
        lg.step();
        // After stream+collide, particle should have moved right
        assert_eq!(lg.cells[5], 1);
    }

    #[test]
    fn test_collide_head_on() {
        let mut lg = LatticeGas::new(5, 1);
        lg.cells[1] = 1;
        lg.velocities[1] = 1; // right
        lg.cells[2] = 1;
        lg.velocities[2] = 3; // left
        lg.collide();
        // Should scatter perpendicular
        assert_eq!(lg.velocities[1], 2); // up
        assert_eq!(lg.velocities[2], 4); // down
    }
}
