import json
import os

MASTER_DATA_PATH = 'kyushu_nanbokucho_data.json'
GAME_DATA_PATH = 'kyushu-wasm-wars/kyushu_data.json'

def apply_all_data():
    if not os.path.exists(MASTER_DATA_PATH) or not os.path.exists(GAME_DATA_PATH):
        print("Error: Files not found.")
        return

    with open(MASTER_DATA_PATH, 'r', encoding='utf-8') as f:
        master_data = json.load(f)
    with open(GAME_DATA_PATH, 'r', encoding='utf-8') as f:
        game_data = json.load(f)

    # 1. Sync Figures
    existing_names = {f['name'] for f in game_data.get('figures', [])}
    for figure in master_data.get('figures', []):
        if figure['name'] not in existing_names:
            game_data['figures'].append(figure)
            existing_names.add(figure['name'])

    # 2. Re-place Initial Placements (Priority based)
    land_tiles = [
        (t['q'], t['r']) for t in game_data.get('map_tiles', []) 
        if t.get('type') != 'sea'
    ]
    
    occupied = set()
    new_placements = []

    zones = {
        "菊池": (4, 4, 3),
        "阿蘇": (5, 3, 2),
        "大宰府": (4, 1, 2),
        "今川": (5, 0, 2),
        "大友": (6, 2, 2),
        "島津": (2, 6, 3),
        "松浦": (1, 1, 2),
        "深堀": (1, 3, 2),
        "相良": (2, 5, 2),
        "肝付": (4, 7, 2),
        "少弐": (3, 1, 2),
    }

    # Priority Groups
    priority_order = [
        # South Core
        "懐良親王", "菊池武光",
        # North Core
        "今川了俊", "今川仲秋", "大友氏泰", "少弐冬資", "島津氏久",
        # Kikuchi 12 Clans
        "赤星氏", "城氏", "隈部氏", "豊田氏", "甲斐氏", "八代氏", "出田氏", "西氏", "内田氏", "木山氏",
        # Important Others
        "五条良長", "足利直冬", "少弐頼尚", "松浦左衛門大夫", "名和顕興", "深堀時広", "阿蘇惟澄"
    ]

    def get_hex_dist(q1, r1, q2, r2):
        return (abs(q1 - q2) + abs(q1 + r1 - q2 - r2) + abs(r1 - r2)) // 2

    placed_names = set()

    for name_query in priority_order:
        fig = next((f for f in game_data['figures'] if name_query in f['name'] and f['name'] not in placed_names), None)
        if not fig: continue
        
        # Determine zone center
        cx, cy, radius = (4, 4, 10)
        fname = fig['name']
        faction = fig['faction']
        
        if "菊池" in fname or "菊池" in faction: cx, cy, radius = zones["菊池"]
        elif "阿蘇" in fname: cx, cy, radius = zones["阿蘇"]
        elif "今川" in fname or "探題" in faction: cx, cy, radius = zones["今川"]
        elif "大友" in fname or "豊後" in faction: cx, cy, radius = zones["大友"]
        elif "島津" in fname or "薩摩" in faction: cx, cy, radius = zones["島津"]
        elif "少弐" in fname: cx, cy, radius = zones["少弐"]
        elif "松浦" in fname: cx, cy, radius = zones["松浦"]
        elif "深堀" in fname: cx, cy, radius = zones["深堀"]
        elif "相良" in fname: cx, cy, radius = zones["相良"]
        elif "肝付" in fname: cx, cy, radius = zones["肝付"]

        best_tile = None
        min_dist = 999
        for tx, ty in land_tiles:
            if (tx, ty) not in occupied:
                dist = get_hex_dist(cx, cy, tx, ty)
                if dist < min_dist:
                    min_dist = dist
                    best_tile = (tx, ty)
        
        if best_tile:
            new_placements.append({"name": fig['name'], "q": best_tile[0], "r": best_tile[1]})
            occupied.add(best_tile)
            placed_names.add(fig['name'])
            # print(f"Placed {fig['name']} at {best_tile}")

    # Fill remaining space with others
    for fig in game_data['figures']:
        if fig['name'] not in placed_names:
            best_tile = None
            min_dist = 999
            for tx, ty in land_tiles:
                if (tx, ty) not in occupied:
                    # Just pick any empty tile
                    best_tile = (tx, ty)
                    break
            
            if best_tile:
                new_placements.append({"name": fig['name'], "q": best_tile[0], "r": best_tile[1]})
                occupied.add(best_tile)
                placed_names.add(fig['name'])
            else:
                break # No more space

    game_data['initial_placements'] = new_placements
    
    with open(GAME_DATA_PATH, 'w', encoding='utf-8') as f:
        json.dump(game_data, f, ensure_ascii=False, indent=4)

    print(f"Successfully re-placed {len(new_placements)} figures.")

if __name__ == "__main__":
    apply_all_data()
