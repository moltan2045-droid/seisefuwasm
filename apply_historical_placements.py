import json
import os

MASTER_DATA_PATH = 'kyushu_nanbokucho_data.json'
GAME_DATA_PATH = 'kyushu-wasm-wars/kyushu_data.json'

def apply_historical_placements():
    if not os.path.exists(GAME_DATA_PATH):
        print(f"Error: {GAME_DATA_PATH} not found.")
        return

    with open(GAME_DATA_PATH, 'r', encoding='utf-8') as f:
        game_data = json.load(f)

    land_tiles = [
        (t['q'], t['r']) for t in game_data.get('map_tiles', []) 
        if t.get('type') != 'sea'
    ]
    
    occupied = set()
    new_placements = []

    # 1. 精密な歴史的拠点マッピング (名前の部分一致で判定)
    historical_map = {
        "懐良親王": (7, 1),      # 大宰府 (征西府全盛期)
        "今川了俊": (6, 0),      # 博多 (探題本陣)
        "菊池武光": (4, 4),      # 隈府 (菊池本拠)
        "阿蘇惟澄": (5, 4),      # 阿蘇
        "大友氏泰": (9, 2),      # 豊後府内
        "島津氏久": (4, 8),      # 鹿児島
        "名和顕興": (3, 5),      # 八代
        "五条良長": (5, 5),      # 矢部 (筑後・肥後国境)
        "松浦左衛門": (1, 1),    # 肥前松浦
        "深堀時広": (1, 3),      # 肥前深堀 (長崎)
        "相良定頼": (3, 6),      # 肥後人吉 (南肥後)
        "肝付兼重": (5, 9),      # 大隅
        "少弐頼尚": (7, 0),      # 筑前 (少弐本領)
        "少弐冬資": (8, 0),      # 筑前
        "一色範氏": (5, 0),      # 博多近郊
    }

    # 2. 菊池十二族の配置エリア (隈府周辺)
    kikuchi_vassals = [
        "赤星", "城氏", "隈部", "豊田", "甲斐", "八代氏", "出田", "西氏", "内田", "木山"
    ]
    kikuchi_center = (4, 4)

    # 優先配置リスト
    placed_names = set()

    def get_hex_dist(q1, r1, q2, r2):
        return (abs(q1 - q2) + abs(q1 + r1 - q2 - r2) + abs(r1 - r2)) // 2

    def find_best_spot(target_q, target_r):
        best_tile = None
        min_dist = 999
        for tx, ty in land_tiles:
            if (tx, ty) not in occupied:
                dist = get_hex_dist(target_q, target_r, tx, ty)
                if dist < min_dist:
                    min_dist = dist
                    best_tile = (tx, ty)
        return best_tile

    # まず特定の重要武将を固定配置
    for pattern, coords in historical_map.items():
        fig = next((f for f in game_data['figures'] if pattern in f['name']), None)
        if fig and fig['name'] not in placed_names:
            spot = find_best_spot(coords[0], coords[1])
            if spot:
                new_placements.append({"name": fig['name'], "q": spot[0], "r": spot[1]})
                occupied.add(spot)
                placed_names.add(fig['name'])
                print(f"Fixed Placement: {fig['name']} at {spot}")

    # 次に菊池十二族を隈府周辺に配置
    for vassal_pattern in kikuchi_vassals:
        fig = next((f for f in game_data['figures'] if vassal_pattern in f['name'] and f['name'] not in placed_names), None)
        if fig:
            spot = find_best_spot(kikuchi_center[0], kikuchi_center[1])
            if spot:
                new_placements.append({"name": fig['name'], "q": spot[0], "r": spot[1]})
                occupied.add(spot)
                placed_names.add(fig['name'])
                print(f"Kikuchi Vassal: {fig['name']} at {spot}")

    # 残りの武将を所属勢力に近い場所に配置
    for fig in game_data['figures']:
        if fig['name'] in placed_names: continue
        
        # デフォルトは中央付近
        target = (4, 4)
        if "北朝" in fig['faction'] or "探題" in fig['faction']: target = (6, 0)
        elif "南朝" in fig['faction'] or "征西" in fig['faction']: target = (4, 4)
        elif "島津" in fig['faction']: target = (4, 8)
        
        spot = find_best_spot(target[0], target[1])
        if spot:
            new_placements.append({"name": fig['name'], "q": spot[0], "r": spot[1]})
            occupied.add(spot)
            placed_names.add(fig['name'])
            print(f"General Placement: {fig['name']} at {spot}")

    game_data['initial_placements'] = new_placements
    
    with open(GAME_DATA_PATH, 'w', encoding='utf-8') as f:
        json.dump(game_data, f, ensure_ascii=False, indent=4)

    print(f"Successfully applied historical placements for {len(new_placements)} units.")

if __name__ == "__main__":
    apply_historical_placements()
