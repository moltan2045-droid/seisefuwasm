use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Faction { South, North, Independent }

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
    pub has_acted: bool,
}

#[wasm_bindgen]
pub struct GameState {
    units: Vec<Unit>,
    pub cursor_q: i32,
    pub cursor_r: i32,
    pub selected_unit_idx: i32, // Optionの代わりに-1を使用
    pub year: i32,
    pub turn: Faction,
    log: String,
}

#[wasm_bindgen]
impl GameState {
    #[wasm_bindgen(constructor)]
    pub fn new() -> GameState {
        GameState {
            units: vec![
                Unit { name: "懐良親王".into(), faction: Faction::South, q: 4, r: 0, hp: 80, max_hp: 80, atk: 15, has_acted: false },
                Unit { name: "今川了俊".into(), faction: Faction::North, q: 4, r: -3, hp: 70, max_hp: 70, atk: 20, has_acted: false },
                Unit { name: "菊池武光".into(), faction: Faction::South, q: 3, r: 2, hp: 100, max_hp: 100, atk: 35, has_acted: false },
                Unit { name: "島津氏久".into(), faction: Faction::Independent, q: 1, r: 5, hp: 80, max_hp: 80, atk: 25, has_acted: false },
            ],
            cursor_q: 4,
            cursor_r: 0,
            selected_unit_idx: -1,
            year: 1338,
            turn: Faction::South,
            log: "南朝軍のターンです。懐良親王を指揮してください。".into(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn log(&self) -> String {
        self.log.clone()
    }


    pub fn is_land(q: i32, r: i32) -> bool {
        match r {
            -4 => q >= 5 && q <= 6,   // 山口北部
            -3 => q >= 4 && q <= 5,   // 山口 (赤間関)
            -2 => q >= 2 && q <= 4,   // 対馬・壱岐
            -1 => false,              // ★関門海峡 (常に海)
            0 => q >= 3 && q <= 6,    // 北九州 (博多・大宰府)
            1 => q >= 1 && q <= 7,    // 肥前・筑後・豊後
            2 => q >= 1 && q <= 6,    // 肥後
            3 => q >= 1 && q <= 5,    // 肥後南部
            4 => q >= 1 && q <= 5,    // 薩摩・大隅
            5 => q >= 1 && q <= 4,    // 薩摩
            6 => q >= 2 && q <= 3,    // 南端
            _ => false,
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
            "t" => {
                self.turn = if self.turn == Faction::South { Faction::North } else { Faction::South };
                for u in &mut self.units { u.has_acted = false; }
                self.log = format!("{:?}のターンになりました。", self.turn);
                self.selected_unit_idx = -1;
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
                            let damage = self.units[selected_idx].atk;
                            self.units[target_unit_idx].hp -= damage;
                            self.log = format!("{}が{}に{}ダメージ！", self.units[selected_idx].name, self.units[target_unit_idx].name, damage);
                            if self.units[target_unit_idx].hp <= 0 {
                                self.log += &format!(" {}は討ち取られました！", self.units[target_unit_idx].name);
                                self.units.remove(target_unit_idx);
                            }
                            self.units.get_mut(selected_idx).map(|u| u.has_acted = true);
                            self.selected_unit_idx = -1;
                        }
                    } else if GameState::is_land(target_q, target_r) {
                        // 移動
                        self.units[selected_idx].q = target_q;
                        self.units[selected_idx].r = target_r;
                        self.units[selected_idx].has_acted = true;
                        self.log = format!("{}が移動しました。", self.units[selected_idx].name);
                        self.selected_unit_idx = -1;
                    }
                } else if let Some(idx) = clicked_unit_idx {
                    if self.units[idx].faction == self.turn && !self.units[idx].has_acted {
                        self.selected_unit_idx = idx as i32;
                        self.log = format!("{}を選択中...", self.units[idx].name);
                    }
                }
            }
            _ => {}
        }
    }
}
