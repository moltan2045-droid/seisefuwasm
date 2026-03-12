import json
import os

def update_hokucho_vassals():
    file_path = 'kyushu_nanbokucho_data.json'
    if not os.path.exists(file_path):
        print(f"Error: {file_path} not found.")
        return

    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)

    # 1. 北朝・幕府方の武将データ
    vassals = [
        {
            "name": "今川仲秋 (いまがわ なかあき)",
            "faction": "北朝 (九州探題)",
            "role": "了俊の弟・肥前守護代",
            "unit_stats": {"type": "探題代官", "hp": 75, "atk": 25, "def": 22, "mov": 4, "rng": 1},
            "special_skill": "水軍の指揮 (水上での攻撃力+20%)",
            "actions": ["兄・了俊と共に九州入り。", "肥前・松浦党の調略を担当。", "水軍を率いて博多湾を押さえる。"],
            "stats_hint": "水陸両用で戦える万能型ユニット。"
        },
        {
            "name": "大友氏泰 (おおとも うじやす)",
            "faction": "北朝 (豊後守護)",
            "role": "豊後守護・北朝の重鎮",
            "unit_stats": {"type": "豊後弓騎兵", "hp": 85, "atk": 28, "def": 18, "mov": 4, "rng": 2},
            "special_skill": "豊後の強弓 (遠距離攻撃の射程+1)",
            "actions": ["豊後を拠点に北朝方として活動。", "菊池氏と激しく争う。"],
            "stats_hint": "強力な遠距離攻撃を持つ支援型ユニット。"
        },
        {
            "name": "少弐冬資 (しょうに フユスケ)",
            "faction": "北朝 (少弐氏)",
            "role": "少弐氏当主・悲劇の将",
            "unit_stats": {"type": "名門武士", "hp": 80, "atk": 30, "def": 20, "mov": 3, "rng": 1},
            "special_skill": "名門の誇り (周囲の味方の士気を高める)",
            "actions": ["少弐頼尚の跡を継ぐ。", "水島の変で今川了俊に暗殺される。"],
            "stats_hint": "平均的な能力だが、イベントで重要になるユニット。"
        },
        {
            "name": "島津氏久 (しまづ うじひさ)",
            "faction": "北朝 (薩摩守護) / 独立",
            "role": "薩摩守護・島津氏総領",
            "unit_stats": {"type": "薩摩隼人", "hp": 90, "atk": 35, "def": 15, "mov": 4, "rng": 1},
            "special_skill": "釣り野伏せ (反撃時のダメージ2倍)",
            "actions": ["当初は北朝方だったが、水島の変以降、今川了俊と対立。", "南九州で独自の勢力を築く。"],
            "stats_hint": "攻撃力に特化した強力なアタッカー。"
        },
        {
            "name": "深堀時広 (ふかほり ときひろ)",
            "faction": "北朝 (肥前国人)",
            "role": "長崎港管理者・水軍",
            "unit_stats": {"type": "海賊大名", "hp": 70, "atk": 24, "def": 16, "mov": 5, "rng": 1},
            "special_skill": "海上の狼 (海上での移動コスト1、攻撃力+10)",
            "actions": ["長崎・深堀を拠点とする。", "倭寇討伐や海上警備で活躍。"],
            "stats_hint": "海上戦に特化したユニット。"
        }
    ]

    # 2. データ反映
    existing_names = {f['name'] for f in data.get('figures', [])}
    added_count = 0
    
    for v in vassals:
        if v['name'] not in existing_names:
            data['figures'].append(v)
            added_count += 1
            print(f"追加: {v['name']}")
        else:
            # 既存データの更新も考慮する場合はここで処理
            pass

    # 3. メモ追加
    memos = [
        {
            "topic": "水島の変 (みずしまのへん)",
            "content": "1375年、今川了俊が少弐冬資を酒宴の席で暗殺した事件。これにより島津氏久が激怒して帰国し、九州探題による統制が崩壊するきっかけとなった。北朝方の結束に致命的な亀裂を入れた事件。",
            "timestamp": "2026-03-10 13:00:00"
        }
    ]
    
    for m in memos:
        if not any(dm['topic'] == m['topic'] for dm in data.get('memos', [])):
            data['memos'].append(m)
            print(f"メモ追加: {m['topic']}")

    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, indent=4)
    
    print(f"北朝武将データの反映完了 (追加: {added_count}件)")

if __name__ == "__main__":
    update_hokucho_vassals()
