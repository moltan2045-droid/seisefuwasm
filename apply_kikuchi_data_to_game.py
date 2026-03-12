import json
import os

MASTER_DATA_PATH = 'kyushu_nanbokucho_data.json'
GAME_DATA_PATH = 'kyushu-wasm-wars/kyushu_data.json'

def apply_kikuchi_data():
    if not os.path.exists(MASTER_DATA_PATH) or not os.path.exists(GAME_DATA_PATH):
        print("Error: Files not found.")
        return

    # Load Master Data
    with open(MASTER_DATA_PATH, 'r', encoding='utf-8') as f:
        master_data = json.load(f)

    # Load Game Data
    with open(GAME_DATA_PATH, 'r', encoding='utf-8') as f:
        game_data = json.load(f)

    # 1. Extract Kikuchi Clans from Master Data
    kikuchi_clans = [
        f for f in master_data.get('figures', [])
        if "菊池" in f.get('faction', '') and f['name'] != "菊池武光" # 武光は既にいるはず
    ]

    print(f"Found {len(kikuchi_clans)} Kikuchi clan members in Master Data.")

    # 2. Add to Game Data (figures)
    existing_names = {f['name'] for f in game_data.get('figures', [])}
    added_count = 0
    
    for clan in kikuchi_clans:
        if clan['name'] not in existing_names:
            game_data['figures'].append(clan)
            existing_names.add(clan['name'])
            added_count += 1
            print(f"Added figure: {clan['name']}")
        else:
            # Update existing? (Optional)
            pass

    print(f"Total figures added: {added_count}")

    # 3. Set Initial Placements
    # Kikuchi Takemitsu is likely at (4, 4) or similar. 
    # Let's place the 12 clans around Higo (central Kyushu).
    # Higo coords roughly: q=3~5, r=3~5
    
    # Check existing placements
    occupied = {(p['q'], p['r']) for p in game_data.get('initial_placements', [])}
    
    # Potential spots for Kikuchi vassals (relative to Kikuchi base)
    # Trying to place them in a ring around the center
    potential_spots = [
        (3, 4), (5, 3), (4, 3), (4, 5), (3, 5), (5, 4), # Inner ring
        (2, 4), (6, 3), (3, 3), (5, 5), (2, 5), (6, 4)  # Outer ring
    ]
    
    placed_count = 0
    for clan in kikuchi_clans:
        # Check if already placed
        if any(p['name'] == clan['name'] for p in game_data.get('initial_placements', [])):
            continue
            
        # Find a spot
        spot_found = False
        for q, r in potential_spots:
            if (q, r) not in occupied:
                game_data['initial_placements'].append({
                    "name": clan['name'],
                    "q": q,
                    "r": r
                })
                occupied.add((q, r))
                print(f"Placed {clan['name']} at ({q}, {r})")
                spot_found = True
                placed_count += 1
                break
        
        if not spot_found:
            print(f"Warning: No space for {clan['name']}")

    # Save Game Data
    with open(GAME_DATA_PATH, 'w', encoding='utf-8') as f:
        json.dump(game_data, f, ensure_ascii=False, indent=4)

    print("Successfully applied Kikuchi data to game.")

if __name__ == "__main__":
    apply_kikuchi_data()
