import json
import os

def update_kikuchi_vassals_extended():
    file_path = 'kyushu_nanbokucho_data.json'
    if not os.path.exists(file_path):
        print(f"Error: {file_path} not found.")
        return

    with open(file_path, 'r', encoding='utf-8') as f:
        data = json.load(f)

    # 1. 菊池十二族の定義
    clans = [
        {
            "name": "赤星氏 (あかほし)",
            "faction": "南朝 (征西府 / 菊池三家)",
            "role": "菊池一門重鎮・軍事指揮官",
            "unit_stats": {"hp": 90, "atk": 32, "def": 22, "mov": 4, "rng": 1},
            "special_skill": "赤星の猛攻 (隣接する味方「菊池」ユニット1体につきATK+5)",
            "actions": ["菊池三家の一つ。菊池本家を軍事面で支える最強の庶流。", "筑後川の戦いでは主力として北朝軍を圧倒。"],
            "stats_hint": "菊池軍の攻撃の核となる準エースユニット。"
        },
        {
            "name": "城氏 (じょう)",
            "faction": "南朝 (征西府 / 菊池三家)",
            "role": "菊池一門重鎮・防衛指揮官",
            "unit_stats": {"hp": 90, "atk": 25, "def": 32, "mov": 3, "rng": 1},
            "special_skill": "城村の鉄壁 (城・拠点にいる際、被ダメージを40%軽減)",
            "actions": ["菊池三家の一つ。城村城を拠点に菊池本拠地の防衛を担う。", "一族の合議制「寄合衆」の中心的メンバー。"],
            "stats_hint": "極めて高い防御力を持ち、拠点を死守する守備の要。"
        },
        {
            "name": "隈部氏 (くまべ)",
            "faction": "南朝 (征西府 / 菊池三家)",
            "role": "菊池一門重鎮・実力派国人",
            "unit_stats": {"hp": 85, "atk": 28, "def": 25, "mov": 4, "rng": 1},
            "special_skill": "隈部の意地 (HPが30%以下の時、全ステータス+10)",
            "actions": ["菊池三家の一つ。肥後北部に強い勢力を持つ。", "菊池氏衰退後も地域の実力者として長く存続した。"],
            "stats_hint": "バランスが良く、土壇場での粘り強さを持つユニット。"
        },
        {
            "name": "豊田氏 (とよだ)",
            "faction": "南朝 (征西府 / 菊池一族)",
            "role": "武光の出自・精鋭部隊",
            "unit_stats": {"hp": 80, "atk": 35, "def": 20, "mov": 5, "rng": 1},
            "special_skill": "十郎の突撃 (移動距離が3マス以上の時、ATK+10)",
            "actions": ["15代当主・菊池武光（豊田十郎）を輩出した一族。", "武光の直属部隊として、神速の機動力を誇る。"],
            "stats_hint": "高い攻撃力と機動力を併せ持つ、奇襲特化ユニット。"
        },
        {
            "name": "甲斐氏 (かい)",
            "faction": "南朝 (征西府 / 菊池一族)",
            "role": "菊池氏軍師・外交担当",
            "unit_stats": {"hp": 70, "atk": 20, "def": 20, "mov": 4, "rng": 2},
            "special_skill": "阿蘇への橋渡し (隣接する味方「阿蘇」ユニットのMOV+1)",
            "relationships": [{"target": "阿蘇惟澄", "type": "盟友", "score": 90}],
            "actions": ["元は足利派であったが、後に菊池氏の知恵袋となる。", "阿蘇氏との連携を主導し、肥後東部の安定に寄与。"],
            "stats_hint": "遠距離攻撃と味方へのバフを持つサポート型ユニット。"
        },
        {
            "name": "八代氏 (やつしろ)",
            "faction": "南朝 (征西府 / 菊池一族)",
            "role": "肥後南部守護・港湾防衛",
            "unit_stats": {"hp": 85, "atk": 24, "def": 26, "mov": 3, "rng": 1},
            "special_skill": "不知火の守り (水辺タイルに隣接している際、DEF+15)",
            "actions": ["重要拠点・八代を拠点とする一族。", "南九州の北朝勢力に対する防波堤の役割を果たす。"],
            "stats_hint": "水辺付近での防衛に強みを発揮するユニット。"
        },
        {
            "name": "出田氏 (いでた)",
            "faction": "南朝 (征西府 / 菊池一族)",
            "role": "菊池十八外城・先鋭部隊",
            "unit_stats": {"hp": 75, "atk": 30, "def": 18, "mov": 4, "rng": 1},
            "special_skill": "外城の尖兵 (敵領地内での攻撃時、ATK+8)",
            "actions": ["出田城主。南北朝の激戦期に常に最前線に配置された。", "菊池家憲を遵守し、一族の結束を体現。"],
            "stats_hint": "攻勢時に高いボーナスを得るアタッカーユニット。"
        },
        {
            "name": "西氏 (にし)",
            "faction": "南朝 (征西府 / 菊池一族)",
            "role": "菊池本家近衛・忠節",
            "unit_stats": {"hp": 80, "atk": 22, "def": 28, "mov": 3, "rng": 1},
            "special_skill": "西寺の静寂 (隣接する敵のスキル発動率を低下させる)",
            "actions": ["菊池郡西寺を拠点とし、本家と生死を共にした忠義の族。", "常に本陣の護衛を任された。"],
            "stats_hint": "敵の妨害を得意とする守備的近衛ユニット。"
        },
        {
            "name": "内田氏 (うちだ)",
            "faction": "南朝 (征西府 / 菊池一族)",
            "role": "山鹿郡守備・遊撃部隊",
            "unit_stats": {"hp": 75, "atk": 26, "def": 22, "mov": 5, "rng": 1},
            "special_skill": "山鹿の山駆け (林・山岳タイルでの移動コスト無視)",
            "actions": ["山鹿郡内田を拠点とする。地元の険しい地形を利用した戦いを得意とした。"],
            "stats_hint": "地形を無視した機動が可能な遊撃ユニット。"
        },
        {
            "name": "木山氏 (きやま)",
            "faction": "南朝 (征西府 / 菊池一族)",
            "role": "阿蘇・益城方面防壁",
            "unit_stats": {"hp": 80, "atk": 24, "def": 24, "mov": 4, "rng": 1},
            "special_skill": "益城の盾 (周囲2マスの「山城」の防御ボーナスを2倍にする)",
            "actions": ["益城郡木山を拠点に、阿蘇氏との境界線を守る。"],
            "stats_hint": "地形効果を増幅させる防衛サポートユニット。"
        }
    ]

    # 2. 既存の figures を更新
    existing_names = {f['name'] for f in data.get('figures', [])}
    for clan in clans:
        if clan['name'] not in existing_names:
            data['figures'].append(clan)
            print(f"追加: {clan['name']}")
        else:
            # 既存のデータを更新（パラメータなどを最新化）
            for i, f in enumerate(data['figures']):
                if f['name'] == clan['name']:
                    data['figures'][i].update(clan)
                    print(f"更新: {clan['name']}")

    # 3. メモの追加
    memos_to_add = [
        {
            "topic": "菊池家憲 (きくちかけん)",
            "content": "13代当主・菊池武重が定めた、一族の団結を法的に裏付ける文書。当主の独裁を禁じ、一族の有力者（寄合衆）の血判署名による合議制を確立した。これが南北朝の過酷な戦いの中でも、菊池十二族の離反を防いだ最大の要因である。",
            "timestamp": "2026-03-10 12:10:00"
        },
        {
            "topic": "菊池十八外城 (きくちじゅうはちそとじろ)",
            "content": "本城である隈府を守るため、周囲の要衝に配置された18の支城ネットワーク。赤星城、城村城、出田城などに十二族を配置し、有機的な連携による多層防御を実現していた。この防衛網により、菊池は幾度もの侵攻を耐え抜いた。",
            "timestamp": "2026-03-10 12:15:00"
        }
    ]

    for m in memos_to_add:
        if not any(dm['topic'] == m['topic'] for dm in data.get('memos', [])):
            data['memos'].append(m)
            print(f"メモ追加: {m['topic']}")

    with open(file_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, ensure_ascii=False, indent=4)
    print("菊池十二族の全データ反映が完了しました。")

if __name__ == "__main__":
    update_kikuchi_vassals_extended()
