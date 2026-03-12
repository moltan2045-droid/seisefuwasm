import json
import os

GAME_DATA_PATH = 'kyushu-wasm-wars/kyushu_data.json'

def expand_map():
    if not os.path.exists(GAME_DATA_PATH):
        print(f"Error: {GAME_DATA_PATH} not found.")
        return

    with open(GAME_DATA_PATH, 'r', encoding='utf-8') as f:
        data = json.load(f)

    # 既存のタイルを削除し、一から生成（九州全体を表現）
    new_tiles = []
    
    # 座標範囲の定義 (Q: 0~10, R: -2~10)
    # 地形の定義: plain, mountain, forest, sea
    
    # 基本は全て海
    map_dict = {}

    def set_tile(q, r, t_type):
        map_dict[(q, r)] = t_type

    # 1. 筑前・筑後 (北部九州)
    for q in range(4, 9):
        for r in range(0, 3):
            set_tile(q, r, "plain")
    set_tile(7, 1, "plain") # 大宰府周辺

    # 2. 肥前 (西部九州)
    for q in range(1, 4):
        for r in range(1, 5):
            set_tile(q, r, "plain")
    set_tile(2, 2, "mountain") # 肥前の山

    # 3. 肥後 (中部九州 - 菊池本拠地)
    for q in range(3, 7):
        for r in range(3, 6):
            set_tile(q, r, "plain")
    set_tile(4, 4, "plain") # 菊池
    set_tile(5, 4, "mountain") # 阿蘇山

    # 4. 豊後 (東部九州)
    for q in range(8, 11):
        for r in range(1, 5):
            set_tile(q, r, "plain")
    set_tile(9, 3, "mountain") # 豊後の山

    # 5. 日向・大隅・薩摩 (南部九州)
    for q in range(3, 8):
        for r in range(6, 11):
            set_tile(q, r, "plain")
    set_tile(4, 8, "plain") # 鹿児島湾周辺
    set_tile(5, 7, "mountain") # 霧島周辺

    # 6. 海の境界（陸地の周りを少し埋める）
    # 全体の矩形範囲を sea で埋めてから陸地を上書き
    final_tiles = []
    for q in range(0, 12):
        for r in range(-2, 12):
            t_type = map_dict.get((q, r), "sea")
            final_tiles.append({"q": q, "r": r, "type": t_type})

    data['map_tiles'] = final_tiles
    
    # 拠点の座標も更新
    data['key_locations'] = [
        {"name": "大宰府", "coords": {"q": 7, "r": 1}, "terrain": "plain", "bonus": {"def": 10, "atk": 5, "recovery": 10}},
        {"name": "博多", "coords": {"q": 6, "r": 0}, "terrain": "plain", "bonus": {"def": 5, "atk": 10, "recovery": 15}},
        {"name": "隈府 (菊池)", "coords": {"q": 4, "r": 4}, "terrain": "plain", "bonus": {"def": 15, "atk": 5, "recovery": 20}},
        {"name": "阿蘇", "coords": {"q": 5, "r": 4}, "terrain": "mountain", "bonus": {"def": 20, "atk": 0, "recovery": 10}},
        {"name": "府内 (豊後)", "coords": {"q": 9, "r": 2}, "terrain": "plain", "bonus": {"def": 10, "atk": 5, "recovery": 15}},
        {"name": "鹿児島", "coords": {"q": 4, "r": 8}, "terrain": "plain", "bonus": {"def": 10, "atk": 5, "recovery": 15}},
        {"name": "八代", "coords": {"q": 3, "r": 5}, "terrain": "plain", "bonus": {"def": 10, "atk": 0, "recovery": 10}},
        {"name": "矢部", "coords": {"q": 5, "r": 5}, "terrain": "mountain", "bonus": {"def": 25, "atk": 0, "recovery": 20}}
    ]

    with open(GAME_DATA_PATH, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, indent=4)

    print(f"Map expanded to {len(final_tiles)} tiles (including sea).")

if __name__ == "__main__":
    expand_map()
