use std::array;

use arrayvec::ArrayString;

use crate::{Float, PaperClips, combat::{battle_name::{BATTLE_NAMES, BattleName, BattleNumbers, max_battlename_len}, ship::{Ship, Status, Team}}, project::{PROJECT_121, PROJECT_134}, rng::PCRng};

pub mod ship;
pub mod battle_name;

/// # battleGRID_WIDTH
pub const BATTLE_WIDTH: usize = 310;
/// # battleGRID_HEIGHT
pub const BATTLE_HEIGHT: usize = 150;
/// # battleGRID_WIDTH
pub const GRID_WIDTH: usize = BATTLE_WIDTH / GRID_SCALE; // 31
/// # battleGRID_HEIGHT
pub const GRID_HEIGHT: usize = BATTLE_HEIGHT / GRID_SCALE; // 15

pub const GRID_SCALE: usize = 10;
pub const PROBE_COMBAT_BASE_RATE: Float = 0.15;
pub const DEATH_THRESHOLD: Float = 0.5;

pub const DEFAULT_BATTLENAME: &str = "Durenstein 1";
pub const THRENODY_START: &str = "Threnody for the Heroes of ";
// max battle name + space + max number base 10 (+1 for ceiling)
pub const MAX_BATTLENAME_LEN: usize = max_battlename_len()+1 + 1+BattleID::MAX.ilog10() as usize;
pub const MAX_THRENODY_LEN: usize = THRENODY_START.len() + MAX_BATTLENAME_LEN;
pub const WAR_TRIGGER: Float = 1000000.0;

pub type BattleID = u32;
pub type Grid = [[Cell; GRID_HEIGHT]; GRID_WIDTH];
pub type ShipCount = u8;
pub type Honor = u64;

#[derive(Debug, Clone)]
pub struct Combat {
    pub battle_flag: bool,

    pub battle_name_flag: bool,

    /// # battles
    /// originally was an array with at max 1 value and only checks `battles.length >= 1`, so a bool is fine
    pub battles: bool,

    pub ships: Vec<Ship>,
    /// # (numLeftShips, numRightShips)
    pub ship_count: (ShipCount, ShipCount),
    /// # (battleLEFTSHIPS, battleRIGHTSHIPS)
    pub max_ships: (ShipCount, ShipCount),
    pub attack_speed_flag: bool,
    pub unit_size: Float,

    pub battle_clock: u16,
    pub battle_end_delay: u8,
    pub battle_end_timer: u8,
    pub master_battle_clock: u16,

    /// # honor
    pub honor: Honor,
    pub honor_count: bool,
    pub honor_reward: Honor,
    pub bonus_honor: Honor,

    pub battle_name: BattleName,
    pub battle_numbers: BattleNumbers,
    /// # threnodyTitle
    pub threnody_title: ArrayString<MAX_BATTLENAME_LEN>,
    pub threnody_project: ArrayString<MAX_BATTLENAME_LEN>,
}

impl Default for Combat {
    fn default() -> Self {
        Self {
            battle_flag: false,

            battle_name_flag: false,

            battles: false,

            ships: Vec::new(),
            ship_count: (0, 0),
            max_ships: (200, 200),
            attack_speed_flag: false,
            unit_size: 0.0,

            battle_clock: 0,
            battle_end_delay: 0,
            battle_end_timer: 100,
            master_battle_clock: 0,

            honor: 0,
            honor_count: false,
            honor_reward: 0,
            bonus_honor: 0,

            battle_name: BattleName::Id(0),
            battle_numbers: [0; BATTLE_NAMES.len()],
            threnody_title: ArrayString::from(DEFAULT_BATTLENAME).expect("Always valid"),
            threnody_project: ArrayString::from(DEFAULT_BATTLENAME).expect("Always valid"),

        }
    }
}

impl PaperClips {
    pub fn update_combat(&mut self) {
        // clear_frame(); // handled by GUI
        let mut grid = self.combat.create_grid();
        self.combat.move_ships(&grid);
        self.do_combat(&mut grid);
    }
    #[inline]
    pub fn war(&mut self) {
        self.check_for_battles();
    }
    pub fn check_for_battles(&mut self) {
        if self.space.drifter_count > WAR_TRIGGER && self.space.probe_count > 0.0 && !self.combat.battles {
            if self.rng.random_bool(0.5, true) {
                self.combat.battle_flag = true;
                self.create_battle();
            }
        }
    }
    pub fn create_battle(&mut self) {
        let combat = &mut self.combat;
        let space = &mut self.space;
        let rng = &mut self.rng;

        combat.unit_size = (space.probe_count.min(space.drifter_count) / 100.0).max(1.0);

        let rr = (rng.random_float(false) * space.drifter_count).max(1.0);
        let ss = (rng.random_float(true) * space.probe_count).max(1.0);
        // let tt = rng.random_float(true) * space.available_matter;

        // battleLEFTSHIPS
        combat.max_ships.0 = (ss / 1000000.0).ceil().min(200.0) as u8;
        if combat.max_ships.0 >= 200 {
            if rng.random_bool(0.5, false) {
                combat.max_ships.0 = (rng.random_float(true) * 175.0).ceil() as ShipCount;
            }
        }
        // battleRIGHTSHIPS
        combat.max_ships.1 = (rr / 1000000.0).ceil().min(200.0) as u8;

        combat.update_battle_name(rng);
    }
    pub fn check_for_battle_end(&mut self) {
        let combat = &mut self.combat;

        if !combat.battles { return }

        if combat.ship_count.0 == 0 || combat.ship_count.1 == 0 {
            if self.projects.is_active(PROJECT_121) {
                match combat.ship_count {
                    (0, _) => { // LOST
                        if !combat.honor_count {
                            combat.bonus_honor = 0;
                            combat.honor -= combat.max_ships.0 as Honor;
                            combat.honor_count = true;
                        }
                        if let BattleName::Name(name) = combat.battle_name {
                            combat.threnody_title = name;
                        }
                    }
                    (_, 0) => { // WIN
                        if !combat.honor_count {
                            let honor_reward = combat.max_ships.1 as Honor + combat.bonus_honor;
                            combat.honor += honor_reward;
                        }
                        if self.projects.is_active(PROJECT_134) {
                            combat.bonus_honor += 10;
                        }
                        combat.honor_count = true;
                    }
                    _ => unreachable!()
                }
            }
        } else if combat.ship_count.0 <= 4 || combat.ship_count.1 <= 4 {
            combat.battle_clock += 1;
            if combat.battle_clock > 2000 {
                combat.end_battle();
            }
        }

        if combat.battle_end_delay >= combat.battle_end_timer {
            combat.end_battle();
        }

        combat.master_battle_clock += 1;
        if combat.master_battle_clock >= 8000 {
            combat.end_battle();
        }
    }
    pub fn do_combat(&mut self, grid: &mut Grid) {
        let space = &mut self.space;
        let combat = &mut self.combat;

        let px = space.probe_count * PROBE_COMBAT_BASE_RATE;
        let dx = space.drifter_count;

        let ooda = if combat.attack_speed_flag { space.probe_speed * 0.2 } else { 0.0 };

        for x in 0..GRID_WIDTH {
            for y in 0..GRID_HEIGHT {
                let cell = &mut grid[x][y];

                // First Check if there are enough ships in this cell to do combat
                if cell.ships.len() < 2 { continue }

                let mut teams = (0, 0);

                // Now count how many ships for each team in this cell;
                for s in cell.ships.iter().filter(|s| s.status == Status::Alive) {
                    let team = match s.team {
                        Team::Left => &mut teams.0,
                        Team::Right => &mut teams.1,
                    };
                    *team += 1;
                }
                if teams.0 == 0 || teams.1 == 0 { continue }

                // now we have at least one ship of each team in this cell. 
                // roll a weighted die to see if each ship gets killed

                for s in &mut cell.ships {
                    let (dice_roll, death_threshold) = match s.team {
                        Team::Left => (
                            self.rng.random_float(true) * dx
                                * (teams.1 as Float / teams.0 as Float) * 0.5,
                            DEATH_THRESHOLD + ooda,
                        ),
                        Team::Right => (
                            self.rng.random_float(false) * px + space.probe_count * 0.1
                                * (teams.0 as Float / teams.1 as Float) * 0.5,
                            DEATH_THRESHOLD,
                        ),
                    };

                    if dice_roll > death_threshold {
                        s.status = Status::Dead(0);
                        let (count, collector) = match s.team {
                            Team::Left => {
                                teams.0 -= 1;
                                combat.ship_count.0 -= 1;
                                (&mut space.probe_count, &mut space.probes_lost_combat)
                            }
                            Team::Right => {
                                teams.1 -= 1;
                                combat.ship_count.1 -= 1;
                                (&mut space.drifter_count, &mut space.drifters_killed)
                            }
                        };
                        combat.unit_size = combat.unit_size.min(*count);
                        *count -= combat.unit_size;
                        *collector += combat.unit_size;
                    }
                }
            }
        }
    }
}

impl Combat {
    pub fn find_centroid(&self) -> Pos {
        let mut centroid = self.ships
            .iter()
            .fold(Pos { x: 0.0, y: 0.0 }, |mut c, s| {
                c.x += s.x;
                c.y += s.y;
                c
            });
        let ships_alive = self.ships.len() as Float;
        // normalize
        centroid.x /= ships_alive;
        centroid.y /= ships_alive;
        // give some tendency to center, so they bunch in the middle
        centroid.x = (centroid.x * 0.8) + (BATTLE_WIDTH as Float / 2.0 * 0.2);
        centroid.y = (centroid.y * 0.8) + (BATTLE_HEIGHT as Float / 2.0 * 0.2);
        centroid
    }
    #[inline]
    pub fn end_battle(&mut self) {
        self.honor_count = false;
        self.battle_clock = 0;
        self.master_battle_clock = 0;
        self.battle_end_delay = 0;
        self.battles = false;
    }
    pub fn battle_restart(&mut self, rng: &mut PCRng) {
        self.ship_count = (0, 0);
        self.ships.clear();

        // grid isn't global anymore

        // create ships... alternate left team and right team so there's no advantage
        // for array position

        let mut left_ship_turn = false;

        while self.ship_count.0 < self.max_ships.0 || self.ship_count.1 < self.max_ships.1 {
            if left_ship_turn {
                self.ships.push(Ship::new_rng(Team::Left, rng));
                self.ship_count.0 += 1;
                if self.ship_count.1 < self.max_ships.1 { left_ship_turn = false }
            } else {
                self.ships.push(Ship::new_rng(Team::Right, rng));
                self.ship_count.1 += 1;
                if self.ship_count.0 < self.max_ships.0 { left_ship_turn = true }
            }
        }
    }
    /// # UpdateGrid
    pub fn create_grid(&mut self) -> Grid {
        let mut grid = array::from_fn(|_| array::from_fn(|_| Cell::default()));
        // Update Grid cells with ships in each cell
        for s in &mut self.ships {
            if s.status != Status::Alive { continue }
            // figure out which grid cell the ship is in
            s.gx = ((s.x / GRID_SCALE as Float) as usize).clamp(0, GRID_WIDTH - 1);
            s.gy = ((s.y / GRID_SCALE as Float) as usize).clamp(0, GRID_HEIGHT - 1);
            grid[s.gx][s.gy].ships.push(*s);
        }
        grid
    }
    pub fn move_ships(&mut self, grid: &Grid) {
        let centroid = self.find_centroid();
        self.ships.retain_mut(|s| {
            match s.status {
                Status::Dead(ref mut f) => {
                    // code handled by GUI
                    *f += 1;
                    *f < 10
                }
                Status::Alive => {
                    s.move_ship(grid, &centroid);
                    // code handled by GUI
                    true
                }
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct Pos {
    x: Float,
    y: Float,
}

#[derive(Debug, Clone, Default)]
pub struct Cell {
    pub ships: Vec<Ship>,
}
