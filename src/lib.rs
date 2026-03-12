use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Faction { South, North, Independent }

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Season { Spring, Summer, Autumn, Winter }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerrainType {
    pub name: String,
    pub mov: i32,
    pub def: i32,
    pub color: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub q: i32,
    pub r: i32,
    pub terrain: String,
}

#[wasm_bindgen]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Unit {
    name: String,
    pub faction: Faction,
    pub q: i32,
    pub r: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub atk: i32,
    pub def: i32,
    pub mov: i32,
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
}

#[wasm_bindgen]
pub struct GameState {
    units: Vec<Unit>,
    tiles: HashMap<(i32, i32), String>,
    terrain_types: HashMap<String, TerrainType>,
    locations: HashMap<(i32, i32), Location>,
    pub cursor_q: i32,
    pub cursor_r: i32,
    pub selected_unit_idx: i32,
    pub year: i32,
    pub month: i32,
    pub turn: Faction,
    log: String,
}

#[derive(Deserialize)]
struct InitialData {
    figures: Vec<FigureData>,
    terrain_types: HashMap<String, TerrainType>,
    map_tiles: Vec<MapTileData>,
    key_locations: Vec<LocationData>,
    initial_placements: Vec<PlacementData>,
}

#[derive(Deserialize)]
struct LocationData {
    name: String,
    coords: Coords,
    terrain: String,
    bonus: BonusData,
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
}

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

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
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

    pub fn load_json(&mut self, json_str: &str) -> Result<(), JsValue> {
        let data: InitialData = serde_json::from_str(json_str)
            .map_err(|e| JsValue::from_str(&format!("JSON parse error: {}", e)))?;

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
                    has_acted: false,
                });
            }
        }

        self.log = "九州南北朝戦記へようこそ。データが読み込まれました。".into();
        Ok(())
    }

    // AIターン実行
    pub fn run_ai_turn(&mut self) -> bool {
        if self.turn == Faction::South { return false; }

        let current_faction = self.turn;
        let mut acted = false;

        // まだ行動していない自軍ユニットを探す
        let mut unit_to_act_idx = None;
        for (i, u) in self.units.iter().enumerate() {
            if u.faction == current_faction && !u.has_acted {
                unit_to_act_idx = Some(i);
                break;
            }
        }

        if let Some(idx) = unit_to_act_idx {
            acted = true;
            let u_q = self.units[idx].q;
            let u_r = self.units[idx].r;
            
            // 1. 隣接する敵を探す
            let mut target_idx = None;
            for (i, target) in self.units.iter().enumerate() {
                if target.faction != current_faction {
                    let d = hex_dist(u_q, u_r, target.q, target.r);
                    if d == 1 {
                        target_idx = Some(i);
                        break;
                    }
                }
            }

            if let Some(t_idx) = target_idx {
                // 攻撃
                let damage = self.calculate_damage(idx, t_idx);
                self.units[t_idx].hp -= damage;
                let msg = format!("AI: {}が{}に{}ダメージ！", self.units[idx].name, self.units[t_idx].name, damage);
                self.log = msg;
                if self.units[t_idx].hp <= 0 {
                    self.units.remove(t_idx);
                }
                self.units[idx].has_acted = true;
            } else {
                // 2. 最も近い敵に向かって移動
                let mut closest_enemy_pos = None;
                let mut min_dist = 999;
                for target in &self.units {
                    if target.faction != current_faction {
                        let d = hex_dist(u_q, u_r, target.q, target.r);
                        if d < min_dist {
                            min_dist = d;
                            closest_enemy_pos = Some((target.q, target.r));
                        }
                    }
                }

                if let Some((eq, er)) = closest_enemy_pos {
                    // 移動可能な範囲で最も敵に近いマスを探す
                    let mut best_move = (u_q, u_r);
                    let mut best_dist = min_dist;
                    let mov = self.units[idx].mov;

                    for r in (u_r - mov)..=(u_r + mov) {
                        for q in (u_q - mov)..=(u_q + mov) {
                            if hex_dist(u_q, u_r, q, r) <= mov && self.is_land(q, r) {
                                // 他のユニットがいないかチェック
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
                    self.log = format!("AI: {}が移動しました。", self.units[idx].name);
                } else {
                    self.units[idx].has_acted = true;
                }
            }
        } else {
            // 全ユニット行動終了、ターンを回す
            self.handle_input("t");
        }

        acted
    }

    fn calculate_damage(&self, attacker_idx: usize, target_idx: usize) -> i32 {
        let attacker = &self.units[attacker_idx];
        let target = &self.units[target_idx];
        
        let mut atk = attacker.atk;
        let mut def = target.def;

        // 拠点ボーナス
        if let Some(loc) = self.locations.get(&(attacker.q, attacker.r)) {
            atk += loc.atk_bonus;
        }
        if let Some(loc) = self.locations.get(&(target.q, target.r)) {
            def += loc.def_bonus;
        }

        // 季節ペナルティ
        let is_busy_season = self.month == 5 || self.month == 6 || self.month == 9 || self.month == 10;
        if is_busy_season { atk -= 5; }

        (atk - def / 2).max(1)
    }

    pub fn get_location_count(&self) -> usize {
        self.locations.len()
    }

    pub fn get_location_name_at(&self, idx: usize) -> String {
        let loc = self.locations.values().nth(idx).unwrap();
        loc.name.clone()
    }

    pub fn get_location_q_at(&self, idx: usize) -> i32 {
        let loc = self.locations.values().nth(idx).unwrap();
        loc.q
    }

    pub fn get_location_r_at(&self, idx: usize) -> i32 {
        let loc = self.locations.values().nth(idx).unwrap();
        loc.r
    }

    pub fn get_location_type_at(&self, idx: usize) -> String {
        let loc = self.locations.values().nth(idx).unwrap();
        loc.terrain.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn log(&self) -> String {
        self.log.clone()
    }

    pub fn get_season(&self) -> Season {
        match self.month {
            3..=5 => Season::Spring,
            6..=8 => Season::Summer,
            9..=11 => Season::Autumn,
            _ => Season::Winter,
        }
    }

    pub fn is_land(&self, q: i32, r: i32) -> bool {
        self.tiles.contains_key(&(q, r))
    }

    pub fn get_terrain_name(&self, q: i32, r: i32) -> String {
        if let Some(t_id) = self.tiles.get(&(q, r)) {
            if let Some(t) = self.terrain_types.get(t_id) {
                return t.name.clone();
            }
        }
        "海".to_string()
    }

    pub fn get_terrain_color(&self, q: i32, r: i32) -> String {
        if let Some(t_id) = self.tiles.get(&(q, r)) {
            if let Some(t) = self.terrain_types.get(t_id) {
                return t.color.clone();
            }
        }
        "#1a5276".to_string() // 海の色
    }

    pub fn get_terrain_cost(&self, q: i32, r: i32) -> i32 {
        let t_id = self.tiles.get(&(q, r));
        let base_cost = if let Some(id) = t_id {
            self.terrain_types.get(id).map(|t| t.mov).unwrap_or(1)
        } else {
            1 // 海
        };
        
        // 冬の山岳ペナルティ
        let is_mountain = t_id.map(|id| id == "mountain").unwrap_or(false);
        if self.get_season() == Season::Winter && is_mountain {
            base_cost + 2
        } else {
            base_cost
        }
    }

    pub fn get_unit_count(&self) -> usize {
        self.units.len()
    }

    pub fn get_unit_name(&self, idx: usize) -> String {
        self.units[idx].name.clone()
    }

    pub fn get_unit_faction(&self, idx: usize) -> Faction {
        self.units[idx].faction
    }

    pub fn get_unit_q(&self, idx: usize) -> i32 {
        self.units[idx].q
    }

    pub fn get_unit_r(&self, idx: usize) -> i32 {
        self.units[idx].r
    }

    pub fn get_unit_hp(&self, idx: usize) -> i32 {
        self.units[idx].hp
    }

    pub fn get_unit_max_hp(&self, idx: usize) -> i32 {
        self.units[idx].max_hp
    }

    pub fn get_unit_atk(&self, idx: usize) -> i32 {
        self.units[idx].atk
    }

    pub fn get_unit_def(&self, idx: usize) -> i32 {
        self.units[idx].def
    }

    pub fn is_selected_unit(&self, idx: usize) -> bool {
        self.selected_unit_idx == idx as i32
    }

    pub fn handle_input(&mut self, key: &str) {
        match key {
            "ArrowUp" => self.cursor_r -= 1,
            "ArrowDown" => self.cursor_r += 1,
            "ArrowLeft" => self.cursor_q -= 1,
            "ArrowRight" => self.cursor_q += 1,
            "t" => { // ターン終了
                self.turn = match self.turn {
                    Faction::South => Faction::North,
                    Faction::North => Faction::Independent,
                    Faction::Independent => {
                        self.month += 1;
                        if self.month > 12 {
                            self.month = 1;
                            self.year += 1;
                        }
                        Faction::South
                    }
                };
                for u in &mut self.units { u.has_acted = false; }
                self.log = format!("{}年{}月: {:?}のターンになりました。", self.year, self.month, self.turn);
                self.selected_unit_idx = -1;
            }
            "f" => { // 強行軍
                if self.selected_unit_idx >= 0 {
                    let idx = self.selected_unit_idx as usize;
                    if self.units[idx].hp > 20 {
                        self.units[idx].hp -= 10;
                        self.log = format!("{}が強行軍を敢行！ 兵は疲弊したが移動力が拡張された。", self.units[idx].name);
                    }
                }
            }
            "k" => { // 刈田
                if self.selected_unit_idx >= 0 {
                    let idx = self.selected_unit_idx as usize;
                    let is_harvest = self.month == 9 || self.month == 10;
                    let recovery = if is_harvest { 30 } else { 10 };
                    self.units[idx].hp = (self.units[idx].hp + recovery).min(self.units[idx].max_hp);
                    self.units[idx].has_acted = true;
                    self.selected_unit_idx = -1;
                    self.log = format!("{}が刈田を行い、HPが{}回復した。", self.units[idx].name, recovery);
                }
            }
            "Enter" => {
                let clicked_unit_idx = self.units.iter().position(|u| u.q == self.cursor_q && u.r == self.cursor_r);
                
                if self.selected_unit_idx >= 0 {
                    let selected_idx = self.selected_unit_idx as usize;
                    let target_q = self.cursor_q;
                    let target_r = self.cursor_r;
                    
                    if let Some(target_unit_idx) = clicked_unit_idx {
                        if self.units[target_unit_idx].faction != self.units[selected_idx].faction {
                            // 攻撃
                            let is_busy_season = self.month == 5 || self.month == 6 || self.month == 9 || self.month == 10;
                            let penalty = if is_busy_season { 5 } else { 0 };
                            let damage = (self.units[selected_idx].atk - self.units[target_unit_idx].def / 2 - penalty).max(1);
                            
                            self.units[target_unit_idx].hp -= damage;
                            self.log = format!("{}の攻撃！ {}に{}ダメージ。{}", 
                                self.units[selected_idx].name, self.units[target_unit_idx].name, damage,
                                if is_busy_season { "(農繁期につき精彩を欠く)" } else { "" }
                            );
                            
                            if self.units[target_unit_idx].hp <= 0 {
                                self.log += &format!(" {}は討ち取られました！", self.units[target_unit_idx].name);
                                self.units.remove(target_unit_idx);
                            }
                            self.selected_unit_idx = -1;
                        }
                    } else if self.is_land(target_q, target_r) {
                        // 移動コスト判定
                        let dist = ( (self.units[selected_idx].q - target_q).abs() 
                                   + (self.units[selected_idx].q + self.units[selected_idx].r - target_q - target_r).abs() 
                                   + (self.units[selected_idx].r - target_r).abs() ) / 2;
                        
                        let cost = self.get_terrain_cost(target_q, target_r);
                        if dist <= self.units[selected_idx].mov {
                            self.units[selected_idx].q = target_q;
                            self.units[selected_idx].r = target_r;
                            self.units[selected_idx].has_acted = true;
                            self.log = format!("{}が移動しました。 ({}, 移動コスト: {})", 
                                self.units[selected_idx].name, 
                                self.get_terrain_name(target_q, target_r),
                                cost);
                            self.selected_unit_idx = -1;
                        } else {
                            self.log = "そこまでは移動できません。".into();
                        }
                    }
                } else if let Some(idx) = clicked_unit_idx {
                    if self.units[idx].faction == self.turn && !self.units[idx].has_acted {
                        self.selected_unit_idx = idx as i32;
                        self.log = format!("{}を選択中... [k]で刈田, [f]で強行軍", self.units[idx].name);
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
}
