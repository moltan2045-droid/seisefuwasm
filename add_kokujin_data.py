import json

file_path = 'kyushu-wasm-wars/kyushu_data.json'

new_figures = [
    {
        "name": "蒲池久憲 (かまち ひさのり)",
        "faction": "南朝 (征西府)",
        "role": "筑後国人",
        "unit_stats": {"type": "国人衆", "hp": 75, "atk": 23, "def": 18, "mov": 3, "rng": 1},
        "special_skill": "水郷の利 (川・海地形での防御+10)",
        "relationships": [
            {"target": "懐良親王", "type": "忠義", "score": 85, "description": "宇都宮氏として南朝を支える。"}
        ],
        "actions": ["筑後川の戦いで南朝方として参戦。", "南朝の勢力拡大に貢献。"],
        "stats_hint": "水辺に強い防御型ユニット。"
    },
    {
        "name": "草野永幸 (くさの ながゆき)",
        "faction": "南朝 (征西府)",
        "role": "筑後国人",
        "unit_stats": {"type": "山岳武士", "hp": 80, "atk": 25, "def": 20, "mov": 3, "rng": 1},
        "special_skill": "山岳戦 (山地での攻撃力+15%)",
        "relationships": [
            {"target": "懐良親王", "type": "警護", "score": 90, "description": "親王の親衛隊を務める。"}
        ],
        "actions": ["1353年に北朝から離反し南朝へ。", "激戦地で親王を守り抜く。"],
        "stats_hint": "山地での戦闘能力が高い。"
    },
    {
        "name": "秋月種道 (あきづき たねみち)",
        "faction": "北朝 (幕府)",
        "role": "筑前国人",
        "unit_stats": {"type": "国人衆", "hp": 70, "atk": 22, "def": 18, "mov": 3, "rng": 1},
        "special_skill": "野戦陣地 (平地での防御+5)",
        "relationships": [
            {"target": "少弐頼尚", "type": "従属", "score": 80, "description": "少弐氏に従い行動。"}
        ],
        "actions": ["筑後川の戦いで少弐軍として参戦。", "大軍の一翼を担う。"],
        "stats_hint": "標準的な国人ユニット。"
    },
    {
        "name": "星野保家 (ほしの やすいえ)",
        "faction": "北朝 (幕府) / 後に南朝",
        "role": "筑後国人",
        "unit_stats": {"type": "剛の者", "hp": 85, "atk": 28, "def": 15, "mov": 3, "rng": 1},
        "special_skill": "死兵 (HP30%以下で攻撃力大幅上昇)",
        "relationships": [
            {"target": "少弐頼尚", "type": "協力", "score": 70, "description": "筑後川では北朝として参戦。"}
        ],
        "actions": ["筑後川の戦いで北朝方として奮戦。", "後に南朝の忠臣となる。"],
        "stats_hint": "逆境に強い攻撃型ユニット。"
    },
    {
        "name": "相良定頼 (さがら さだより)",
        "faction": "北朝 (幕府)",
        "role": "肥後人吉領主",
        "unit_stats": {"type": "山岳武士", "hp": 75, "atk": 24, "def": 20, "mov": 3, "rng": 1},
        "special_skill": "球磨の急流 (川地形での移動コスト1)",
        "relationships": [
            {"target": "島津氏久", "type": "警戒", "score": -20, "description": "南九州の覇権を争う。"}
        ],
        "actions": ["人吉・球磨地方を統一。", "南北両朝の間で巧みに立ち回る。"],
        "stats_hint": "川や山での機動力が高い。"
    },
    {
        "name": "肝付兼重 (きもつき かねしげ)",
        "faction": "南朝 (征西府)",
        "role": "大隅国人",
        "unit_stats": {"type": "水軍武士", "hp": 80, "atk": 26, "def": 18, "mov": 4, "rng": 1},
        "special_skill": "大隅の隼人 (海岸でのステータス上昇)",
        "relationships": [
            {"target": "島津氏久", "type": "宿敵", "score": -90, "description": "島津氏の支配に激しく抵抗。"}
        ],
        "actions": ["南九州における南朝の拠点。", "島津氏と長年にわたり抗争。"],
        "stats_hint": "対島津に特化した南朝ユニット。"
    }
]

new_placements = [
    { "name": "蒲池久憲", "q": 2, "r": 2 },
    { "name": "草野永幸", "q": 4, "r": 1 },
    { "name": "秋月種道", "q": 5, "r": 0 },
    { "name": "星野保家", "q": 5, "r": 1 },
    { "name": "相良定頼", "q": 2, "r": 4 },
    { "name": "肝付兼重", "q": 2, "r": 5 }
]

try:
    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)

    # Add figures if not exists
    existing_names = [f['name'] for f in data['figures']]
    for fig in new_figures:
        if fig['name'] not in existing_names:
            data['figures'].append(fig)
            print(f"Added figure: {fig['name']}")

    # Add placements if not exists at that hex
    # Check if hex is occupied
    occupied_hexes = {(p['q'], p['r']) for p in data['initial_placements']}
    
    for p in new_placements:
        if (p['q'], p['r']) not in occupied_hexes:
            data['initial_placements'].append(p)
            occupied_hexes.add((p['q'], p['r']))
            print(f"Added placement: {p['name']} at {p['q']},{p['r']}")
        else:
            print(f"Skipped placement: {p['name']} at {p['q']},{p['r']} (Occupied)")

    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, indent=4)

    print("Successfully updated kyushu_data.json")

except Exception as e:
    print(f"Error: {e}")
