use arrayvec::ArrayString;

use crate::{Float, combat::{BattleID, Combat, MAX_BATTLENAME_LEN}, rng::PCRng};

pub const BATTLE_NAMES: &[&str] = &["Aboukir", "Abensberg", "Acre", "Alba de Tormes", "la Albuera", "Algeciras Bay", "Amstetten", "Arcis-sur-Aube", "Aspern-Essling", "Jena-Auerstedt", "Arcole", "Austerlitz", "Badajoz", "Bailen", "la Barrosa", "Bassano", "Bautzen", "Berezina", "Bergisel", "Borodino", "Burgos", "Bucaco", "Cadiz", "Caldiero", "Castiglione", "Castlebar", "Champaubert", "Chateau-Thierry", "Copenhagen", "Corunna", "Craonne", "Dego", "Dennewitz", "Dresden", "Durenstein", "Eckmuhl", "Elchingen", "Espinosa de los Monteros", "Eylau", "Cape Finisterre", "Friedland", "Fuentes de Onoro", "Gevora River", "Gerona", "Hamburg", "Haslach-Jungingen", "Heilsberg", "Hohenlinden", "Jena-Auerstedt", "Kaihona", "Kolberg", "Landshut", "Leipzig", "Ligny", "Lodi", "Lubeck", "Lutzen", "Marengo", "Maria", "Medellin", "Medina de Rioseco", "Millesimo", "Mincio River", "Mondovi", "Montebello", "Montenotte", "Montmirail", "Mount Tabor", "The Nile", "Novi", "Ocana", "Cape Ortegal", "Orthez", "Pancorbo", "Piave River", "The Pyramids", "Quatre Bras", "Raab", "Raszyn", "Rivoli", "Rolica", "La Rothiere", "Rovereto", "Saalfeld", "Schongrabern", "Salamanca", "Smolensk", "Somosierra", "Talavera", "Tamames", "Trafalgar", "Trebbia", "Tudela", "Ulm", "Valls", "Valmaseda", "Valutino", "Vauchamps", "Vimeiro", "Vitoria", "Wagram", "Waterloo", "Wavre", "Wertingen", "Zaragoza"];

pub type BattleNumbers = [BattleID; BATTLE_NAMES.len()];

#[derive(Debug, Clone, Copy)]
pub enum BattleName {
    Id(BattleID),
    Name(ArrayString<MAX_BATTLENAME_LEN>),
}

impl Combat {
    pub fn update_battle_name(&mut self, rng: &mut PCRng) {
        match &mut self.battle_name {
            BattleName::Id(id ) => *id += 1,
            BattleName::Name(name) => {
                name.clear();

                let index = (rng.random_float_no_best() * BATTLE_NAMES.len() as Float) as usize;
                let new_name = BATTLE_NAMES[index];
                let number = &mut self.battle_numbers[index];
                *number += 1;

                name.push_str(new_name);
                name.push(' ');
                name.push_str(&number.to_string());
            }
        }
    }
}

#[inline]
pub const fn max_battlename_len() -> usize {
    let mut i = 0;
    let mut max = 0;
    while i < BATTLE_NAMES.len() {
        let cur = BATTLE_NAMES[i].len();
        if cur > max {
            max = cur
        }
        i += 1;
    }
    max
}
