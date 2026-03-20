use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Faction { South, North, Independent }

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Season { Spring, Summer, Autumn, Winter }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerrainType {
    pub name: String,
    pub mov: i32,
    pub def: i32,
    pub color: String,
    #[serde(default = "default_supply_cost")]
    pub supply_cost: i32,
}

fn default_supply_cost() -> i32 { 1 }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub q: i32,
    pub r: i32,
    pub terrain: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    pub name: String,
    pub faction: Faction,
    pub q: i32,
    pub r: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub mov: i32,
    pub supply: i32,
    pub max_supply: i32,
    pub has_acted: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub q: i32,
    pub r: i32,
    pub terrain: String,
    pub def_bonus: i32,
    pub atk_bonus: i32,
    pub recovery: i32,
    pub supply_capacity: i32,
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub units: Vec<Unit>,
    pub tiles: HashMap<(i32, i32), String>,
    pub terrain_types: HashMap<String, TerrainType>,
    pub locations: HashMap<(i32, i32), Location>,
    pub cursor_q: i32,
    pub cursor_r: i32,
    pub selected_unit_idx: i32,
    pub year: i32,
    pub month: i32,
    pub turn: Faction,
    pub log: String,
}

#[derive(Deserialize)]
struct InitialData {
    figures: Vec<FigureData>,
    terrain_types: HashMap<String, TerrainType>,
    map_tiles: Vec<MapTileData>,
    key_locations: Vec<LocationData>,
    #[serde(default)]
    initial_placements: Vec<PlacementData>,
}

#[derive(Deserialize)]
struct LocationData {
    name: String,
    coords: Coords,
    terrain: String,
    bonus: BonusData,
    #[serde(default)]
    supply_capacity: i32,
}

#[derive(Deserialize)]
struct Coords { q: i32, r: i32 }

#[derive(Deserialize)]
struct BonusData {
    #[serde(default)] def: i32,
    #[serde(default)] atk: i32,
    #[serde(default)] recovery: i32,
}

#[derive(Deserialize)]
struct FigureData {
    name: String,
    faction: String,
    unit_stats: UnitStats,
}

#[derive(Deserialize)]
struct UnitStats {
    hp: i32,
    atk: i32,
    def: i32,
    mov: i32,
    #[serde(default = "default_max_supply")]
    max_supply: i32,
}

fn default_max_supply() -> i32 { 100 }

#[derive(Deserialize)]
struct MapTileData {
    q: i32,
    r: i32,
    #[serde(rename = "type")]
    tile_type: String,
}

#[derive(Deserialize)]
struct PlacementData {
    name: String,
    q: i32,
    r: i32,
}

fn hex_dist(q1: i32, r1: i32, q2: i32, r2: i32) -> i32 {
    ((q1 - q2).abs() + (q1 + r1 - q2 - r2).abs() + (r1 - r2).abs()) / 2
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            units: Vec::new(),
            tiles: HashMap::new(),
            terrain_types: HashMap::new(),
            locations: HashMap::new(),
            cursor_q: 4,
            cursor_r: 0,
            selected_unit_idx: -1,
            year: 1371,
            month: 8,
            turn: Faction::South,
            log: "初期化待ち...".into(),
        }
    }

    pub fn load_json(&mut self, json_str: &str) -> Result<(), String> {
        let data: InitialData = serde_json::from_str(json_str)
            .map_err(|e| format!("JSON parse error: {}", e))?;

        self.terrain_types = data.terrain_types;
        self.tiles = data.map_tiles.into_iter()
            .map(|t| ((t.q, t.r), t.tile_type))
            .collect();
        
        self.locations = data.key_locations.into_iter()
            .map(|l| ((l.coords.q, l.coords.r), Location {
                name: l.name,
                q: l.coords.q,
                r: l.coords.r,
                terrain: l.terrain,
                def_bonus: l.bonus.def,
                atk_bonus: l.bonus.atk,
                recovery: l.bonus.recovery,
                supply_capacity: l.supply_capacity,
            }))
            .collect();

        self.units.clear();
        for placement in data.initial_placements {
            if let Some(figure) = data.figures.iter().find(|f| f.name.starts_with(&placement.name)) {
                let faction = match figure.faction.as_str() {
                    s if s.contains("南朝") => Faction::South,
                    s if s.contains("北朝") => Faction::North,
                    _ => Faction::Independent,
                };

                self.units.push(Unit {
                    name: figure.name.split(' ').next().unwrap_or(&figure.name).to_string(),
                    faction,
                    q: placement.q,
                    r: placement.r,
                    hp: figure.unit_stats.hp,
                    max_hp: figure.unit_stats.hp,
                    atk: figure.unit_stats.atk,
                    def: figure.unit_stats.def,
                    mov: figure.unit_stats.mov,
                    supply: figure.unit_stats.max_supply,
                    max_supply: figure.unit_stats.max_supply,
                    has_acted: false,
                });
            }
        }

        self.log = "九州南北朝戦記へようこそ。データが読み込まれました。".into();
        Ok(())
    }

    pub fn run_ai_turn(&mut self) -> bool {
        if self.turn == Faction::South { return false; }
        let current_faction = self.turn;
        let mut unit_to_act_idx = None;
        for (i, u) in self.units.iter().enumerate() {
            if u.faction == current_faction && !u.has_acted {
                unit_to_act_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = unit_to_act_idx {
            let u_q = self.units[idx].q;
            let u_r = self.units[idx].r;
            let mut target_idx = None;
            for (i, target) in self.units.iter().enumerate() {
                if target.faction != current_faction {
                    if hex_dist(u_q, u_r, target.q, target.r) == 1 {
                        target_idx = Some(i);
                        break;
                    }
                }
            }

            if let Some(t_idx) = target_idx {
                let damage = (self.units[idx].atk - self.units[t_idx].def / 2).max(1);
                self.units[t_idx].hp -= damage;
                self.log = format!("AI: {}が{}に{}ダメージ！", self.units[idx].name, self.units[t_idx].name, damage);
                if self.units[t_idx].hp <= 0 { self.units.remove(t_idx); }
                self.units[idx].has_acted = true;
            } else {
                let mut closest_enemy = None;
                let mut min_dist = 999;
                for target in &self.units {
                    if target.faction != current_faction {
                        let d = hex_dist(u_q, u_r, target.q, target.r);
                        if d < min_dist {
                            min_dist = d;
                            closest_enemy = Some((target.q, target.r));
                        }
                    }
                }
                if let Some((eq, er)) = closest_enemy {
                    let mut best_move = (u_q, u_r);
                    let mut best_dist = min_dist;
                    let mov = self.units[idx].mov;
                    for r in (u_r - mov)..=(u_r + mov) {
                        for q in (u_q - mov)..=(u_q + mov) {
                            if hex_dist(u_q, u_r, q, r) <= mov && self.tiles.contains_key(&(q, r)) {
                                if !self.units.iter().any(|u| u.q == q && u.r == r) {
                                    let d = hex_dist(q, r, eq, er);
                                    if d < best_dist {
                                        best_dist = d;
                                        best_move = (q, r);
                                    }
                                }
                            }
                        }
                    }
                    self.units[idx].q = best_move.0;
                    self.units[idx].r = best_move.1;
                    self.units[idx].has_acted = true;
                } else {
                    self.units[idx].has_acted = true;
                }
            }
            true
        } else {
            self.handle_input("t");
            false
        }
    }

    pub fn handle_input(&mut self, key: &str) {
        match key {
            "t" => {
                self.turn = match self.turn {
                    Faction::South => Faction::North,
                    Faction::North => Faction::Independent,
                    Faction::Independent => {
                        self.month += 1;
                        if self.month > 12 { self.month = 1; self.year += 1; }
                        Faction::South
                    }
                };
                for i in 0..self.units.len() {
                    if self.units[i].faction == self.turn { self.units[i].has_acted = false; }
                }
                self.log = format!("{}年{}月: {:?}のターン。", self.year, self.month, self.turn);
            }
            "Enter" => {
                let clicked_idx = self.units.iter().position(|u| u.q == self.cursor_q && u.r == self.cursor_r);
                if self.selected_unit_idx >= 0 {
                    let s_idx = self.selected_unit_idx as usize;
                    if let Some(t_idx) = clicked_idx {
                        if self.units[t_idx].faction != self.units[s_idx].faction {
                            let dmg = (self.units[s_idx].atk - self.units[t_idx].def / 2).max(1);
                            self.units[t_idx].hp -= dmg;
                            if self.units[t_idx].hp <= 0 { self.units.remove(t_idx); }
                            self.selected_unit_idx = -1;
                        }
                    } else {
                        let dist = hex_dist(self.units[s_idx].q, self.units[s_idx].r, self.cursor_q, self.cursor_r);
                        if dist <= self.units[s_idx].mov {
                            self.units[s_idx].q = self.cursor_q;
                            self.units[s_idx].r = self.cursor_r;
                            self.units[s_idx].has_acted = true;
                            self.selected_unit_idx = -1;
                        }
                    }
                } else if let Some(idx) = clicked_idx {
                    if self.units[idx].faction == self.turn && !self.units[idx].has_acted {
                        self.selected_unit_idx = idx as i32;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn click_hex(&mut self, q: i32, r: i32) {
        self.cursor_q = q;
        self.cursor_r = r;
        self.handle_input("Enter");
    }

    pub fn to_serializable(&self) -> SerializableGameState {
        SerializableGameState {
            units: self.units.clone(),
            tiles: self.tiles.iter().map(|((q, r), v)| (format!("{},{}", q, r), v.clone())).collect(),
            terrain_types: self.terrain_types.clone(),
            locations: self.locations.iter().map(|((q, r), v)| (format!("{},{}", q, r), v.clone())).collect(),
            cursor_q: self.cursor_q,
            cursor_r: self.cursor_r,
            selected_unit_idx: self.selected_unit_idx,
            year: self.year,
            month: self.month,
            turn: self.turn,
            log: self.log.clone(),
        }
    }
}

#[derive(Serialize)]
pub struct SerializableGameState {
    pub units: Vec<Unit>,
    pub tiles: HashMap<String, String>,
    pub terrain_types: HashMap<String, TerrainType>,
    pub locations: HashMap<String, Location>,
    pub cursor_q: i32,
    pub cursor_r: i32,
    pub selected_unit_idx: i32,
    pub year: i32,
    pub month: i32,
    pub turn: Faction,
    pub log: String,
}
