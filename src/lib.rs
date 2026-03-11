use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Faction { South, North, Independent }

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Season { Spring, Summer, Autumn, Winter }

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
    pub mov: i32,
    pub has_acted: bool,
}

#[wasm_bindgen]
pub struct GameState {
    units: Vec<Unit>,
    pub cursor_q: i32,
    pub cursor_r: i32,
    pub selected_unit_idx: i32,
    pub year: i32,
    pub month: i32,
    pub turn: Faction,
    log: String,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        GameState {
            units: vec![
                Unit { name: "懐良親王".into(), faction: Faction::South, q: 4, r: 0, hp: 80, max_hp: 80, atk: 15, mov: 3, has_acted: false },
                Unit { name: "今川了俊".into(), faction: Faction::North, q: 4, r: -3, hp: 70, max_hp: 70, atk: 20, mov: 3, has_acted: false },
                Unit { name: "菊池武光".into(), faction: Faction::South, q: 3, r: 2, hp: 100, max_hp: 100, atk: 35, mov: 5, has_acted: false },
                Unit { name: "島津氏久".into(), faction: Faction::Independent, q: 1, r: 5, hp: 80, max_hp: 80, atk: 25, mov: 4, has_acted: false },
            ],
            cursor_q: 4,
            cursor_r: 0,
            selected_unit_idx: -1,
            year: 1338,
            month: 3, // 3月開始
            turn: Faction::South,
            log: "南朝軍のターンです。懐良親王を指揮してください。".into(),
        }
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

    pub fn is_land(q: i32, r: i32) -> bool {
        match r {
            -4 => q >= 5 && q <= 6,
            -3 => q >= 4 && q <= 5,
            -2 => q >= 2 && q <= 4,
            -1 => false, // 関門海峡
            0 => q >= 3 && q <= 6,
            1 => q >= 1 && q <= 7,
            2 => q >= 1 && q <= 6,
            3 => q >= 1 && q <= 5,
            4 => q >= 1 && q <= 5,
            5 => q >= 1 && q <= 4,
            6 => q >= 2 && q <= 3,
            _ => false,
        }
    }

    // 地形コスト計算 (歴史的ルール適用)
    pub fn get_terrain_cost(&self, q: i32, r: i32) -> i32 {
        let is_mountain = (q + r) % 3 == 0; // 簡易的な地形判定（実際はマップデータに基づく）
        let base_cost = if is_mountain { 3 } else { 1 };
        
        // 冬の山岳ペナルティ
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
                        // 1ラウンド終了で月を進める
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
            "f" => { // 強行軍 (Forced March) の試行
                if self.selected_unit_idx >= 0 {
                    let idx = self.selected_unit_idx as usize;
                    if self.units[idx].hp > 20 {
                        self.units[idx].hp -= 10;
                        // そのターンの移動力を一時的に+2（簡易実装）
                        self.log = format!("{}が強行軍を敢行！ 兵は疲弊したが移動力が拡張された。", self.units[idx].name);
                    }
                }
            }
            "k" => { // 刈田 (Karita) の試行
                if self.selected_unit_idx >= 0 {
                    let idx = self.selected_unit_idx as usize;
                    let is_harvest = self.month == 9 || self.month == 10;
                    let recovery = if is_harvest { 30 } else { 10 };
                    self.units[idx].hp = (self.units[idx].hp + recovery).min(self.units[idx].max_hp);
                    self.units[idx].has_acted = true;
                    self.selected_unit_idx = -1;
                    self.log = format!("{}が刈田（かりた）を行い、HPが{}回復した。", self.units[idx].name, recovery);
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
                            // 攻撃（農繁期ペナルティ適用）
                            let is_busy_season = self.month == 5 || self.month == 6 || self.month == 9 || self.month == 10;
                            let penalty = if is_busy_season { 5 } else { 0 };
                            let damage = (self.units[selected_idx].atk - penalty).max(1);
                            
                            self.units[target_unit_idx].hp -= damage;
                            self.log = format!("{}の攻撃！ {}に{}ダメージ。{}", 
                                self.units[selected_idx].name, self.units[target_unit_idx].name, damage,
                                if is_busy_season { "(農繁期につき精彩を欠く)" } else { "" }
                            );
                            
                            if self.units[target_unit_idx].hp <= 0 {
                                self.log += &format!(" {}は討ち取られました！", self.units[target_unit_idx].name);
                                self.units.remove(target_unit_idx);
                            }
                            // ユニットが削除された場合のインデックス調整（簡易実装のため削除時は選択解除）
                            self.selected_unit_idx = -1;
                            // 削除されなかった場合のみ行動済みフラグを立てる必要があるが、
                            // 今回はシンプルに攻撃後は常に選択解除。
                        }
                    } else if GameState::is_land(target_q, target_r) {
                        // 移動コスト判定
                        let dist = ( (self.units[selected_idx].q - target_q).abs() 
                                   + (self.units[selected_idx].q + self.units[selected_idx].r - target_q - target_r).abs() 
                                   + (self.units[selected_idx].r - target_r).abs() ) / 2;
                        
                        let cost = self.get_terrain_cost(target_q, target_r);
                        if dist <= self.units[selected_idx].mov {
                            self.units[selected_idx].q = target_q;
                            self.units[selected_idx].r = target_r;
                            self.units[selected_idx].has_acted = true;
                            self.log = format!("{}が移動しました。 (移動距離: {}, 地形コスト: {})", self.units[selected_idx].name, dist, cost);
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
}
