# 九州南北朝戦記 (Tauri V2 + Svelte 版)

日本の南北朝時代の九州を舞台にした、スマホ・デスクトップ対応のヘックスシミュレーションゲーム。

## 🛠️ プロジェクト構造
- `seisefu-mobile/`: フロントエンド (Svelte + TS) および Tauri V2 設定。
- `kyushu_core/`: ゲームのメインロジック (Rust)。GUI 依存がなく、独立してテスト可能。
- `kyushu_data.json`: 武将、地形、初期配置などの統合データ。

## 🚀 クローン後のセットアップ手順

別の PC でクローンした後は、以下のコマンドで環境を整えてください。

### 1. 前提条件の確認
- **Rust**: 1.75 以上推奨 (`rustup` でインストール)
- **Node.js**: v18 以上 (`npm` が必要)
- **OS固有の依存**: Tauri V2 の[Prerequisites](https://v2.tauri.app/start/prerequisites/)に従ってください。

### 2. 依存関係のインストール
```bash
cd seisefu-mobile
npm install
```

### 3. デスクトップでの開発実行 (GUI 環境が必要)
```bash
npm run tauri dev
```

### 4. モバイルでの開発実行 (Android SDK / Xcode が必要)
```bash
npm run tauri android dev
# または
npm run tauri ios dev
```

## 🧠 ゲームロジックの修正
ゲームの戦闘計算や AI 思考を修正する場合は、`kyushu_core/src/lib.rs` を編集してください。
GUI がない環境でも以下のコマンドでロジックのチェックが可能です：
```bash
cd kyushu_core
cargo check
```

## 📜 ライセンス
(C) 2026 九州南北朝戦記 開発チーム
