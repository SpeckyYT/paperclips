use crate::{Float, combat::{BATTLE_HEIGHT, BATTLE_WIDTH, GRID_WIDTH, Grid, Pos}, rng::PCRng};

/// # battleMAXSPEED
pub const MAX_SPEED: Float = 2.0;
pub const CENTROID_ACCEL: Float = 0.001;

#[derive(Debug, Clone, Copy)]
pub struct Ship {
    /// # alive / framesDead
    pub status: Status,
    pub team: Team,
    pub gx: usize,
    pub gy: usize,
    pub x: Float,
    pub y: Float,
    pub vx: Float,
    pub vy: Float,
}

impl Ship {
    pub fn new_rng(team: Team, rng: &mut PCRng) -> Self {
        let x_offset = match team { Team::Left => 0.0, Team::Right => 0.8 };
        let x = (rng.random_float_no_best() * 0.2 + x_offset) * GRID_WIDTH as Float;
        let y = rng.random_float_no_best();
        let vx_dir = match team { Team::Left => 1.0, Team::Right => -1.0 };
        let vx = vx_dir * rng.random_float_no_best() * MAX_SPEED;
        let vy = rng.random_float_no_best() - 0.5;

        Self {
            status: Status::Alive,
            team,
            gx: 0,
            gy: 0,
            x,
            y,
            vx,
            vy,
        }
    }
    pub fn move_ship(&mut self, grid: &Grid, centroid: &Pos) {
        self.vx += (centroid.x - self.x) * CENTROID_ACCEL;
        self.vy += (centroid.y - self.y) * CENTROID_ACCEL;
        
        for y in 0.max(self.gy - 1)..BATTLE_HEIGHT.min(self.gy + 2) {
            for x in 0.max(self.gx - 1)..BATTLE_WIDTH.min(self.gx + 2) {
                let cell = &grid[x][y];
                if cell.ships.len() < 2 { continue }
                let mut teammates_considered: u8 = 0;
                for othership in &cell.ships {
                    if matches!(othership.status, Status::Dead(_)) { continue }
                    
                    if othership.team == self.team {
                        teammates_considered += 1;
                        if teammates_considered > 3 { continue }

                        // mild acceleration to match teammates
                        self.vx += othership.vx * 0.01;
                        self.vy += othership.vy * 0.01;

                        // mild acceleration to get space from teammates
                        self.vx -= (othership.x - self.x) * 0.1;
                        self.vy -= (othership.y - self.y) * 0.1;
                    } else {
                        self.vx += othership.vx * 0.2;
                        self.vy += othership.vy * 0.2;
                        // acceleration toward enemies
                        self.vx += (othership.x - self.x) * 0.2;
                        self.vy += (othership.y - self.y) * 0.2;
                    }
                }
            }
        }

        // limit speed to max
        self.vx = self.vx.clamp(-MAX_SPEED, MAX_SPEED);
        self.vy = self.vy.clamp(-MAX_SPEED, MAX_SPEED);

        // move the ship
        self.x += self.vx;
        self.y += self.vy;

        // bounce off edges
        if self.x > BATTLE_WIDTH as Float {
            self.x = BATTLE_WIDTH as Float;
            self.vx = -MAX_SPEED;
        } else if self.x < 0.0 {
            self.x = 0.0;
            self.vx = MAX_SPEED;
        }
        if self.y > BATTLE_HEIGHT as Float {
            self.y = BATTLE_HEIGHT as Float;
            self.vy = -MAX_SPEED;
        } else if self.y < 0.0 {
            self.y = 0.0;
            self.vy = MAX_SPEED;
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Team {
    Left = 0,
    Right = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Alive,
    Dead(u8),
}
