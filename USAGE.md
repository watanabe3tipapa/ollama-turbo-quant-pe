# USAGE.md - Ollama Turbo Quant

## 概要

Ollama Turbo Quantは、ローカルLLM（Ollama）との対話を支援するWebツールです。\
2つのモードがあります：

| モード | ポート | 説明 |
|--------|--------|------|
| **non-Turbo** | 3000 | Ollamaとの連携のみ |
| **Turbo** | 3001 | Ollama + Turbo-Quantによるレスポンス処理 |

---

## アーキテクチャ

```
┌─────────────────────────────────────────────────────────────────┐
│                         Browser                                   │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐            │
│  │   Prompt     │  │   Preview   │  │   Editor    │            │
│  │   Editor    │  │   (表示)    │  │  (編集)     │            │
│  └──────┬──────┘  └──────▲──────┘  └──────▲──────┘            │
│         │                │                │                      │
│         └────────────────┼────────────────┘                      │
│                          │                                       │
│                    WebSocket / REST                               │
└──────────────────────────┼───────────────────────────────────────┘
                           │
        ┌──────────────────┴──────────────────┐
        │                                     │
        ▼                                     ▼
┌───────────────────┐               ┌───────────────────┐
│  Frontend (Vite) │               │  Frontend-Turbo    │
│  Port: 3000      │               │  Port: 3001       │
└────────┬─────────┘               └────────┬─────────┘
         │                                     │
         │ (REST API)                         │ (REST API)
         │                                     │
         ▼                                     ▼
┌───────────────────┐               ┌───────────────────┐
│ Backend (Axum)   │               │ Backend-Turbo     │
│ Port: 8000        │               │ Port: 8001        │
└────────┬─────────┘               └────────┬─────────┘
         │                                     │
         │                                     │ (turbo-quant)
         │                                     ▼
         │                           ┌───────────────────┐
         │                           │  TurboProcessor   │
         │                           │  - TurboQuantizer │
         │                           │  - PolarQuantizer │
         │                           │  - QjlQuantizer   │
         │                           └───────────────────┘
         │                                     │
         ▼                                     ▼
┌─────────────────────────────────────────────────────────────┐
│                      Ollama Server                           │
│                      Port: 11434                             │
│  - Local LLM inference                                       │
│  - Model management                                         │
└─────────────────────────────────────────────────────────────┘
```

---

## 技術スタック

### Frontend

| 技術 | バージョン | 用途 |
|------|-----------|------|
| **React** | 18.x | UIフレームワーク |
| **TypeScript** | 5.3.x | 型安全なJavaScript |
| **Vite** | 5.x | ビルドツール・Devサーバー |
| **Monaco Editor** | 4.6.x | VS Codeと同じエディタコンポーネント |

### Backend

| 技術 | バージョン | 用途 |
|------|-----------|------|
| **Rust** | 1.75+ | 高速・安全なサーバー言語 |
| **Axum** | 0.7.x | Webフレームワーク |
| **Tokio** | 1.x | 非同期ランタイム |
| **Reqwest** | 0.11.x | HTTPクライアント（Ollama通信） |

### Turbo-Quant（backend-turboのみ）

| 技術 | 用途 |
|------|------|
| **turbo-quant** | [RecursiveIntell/turbo-quant](https://github.com/RecursiveIntell/turbo-quant) - ベクトル量子化ライブラリ |
| **nalgebra** | 0.32 - 数値計算ライブラリ |
| **rand** | 0.8 - 乱数生成 |

---

## プロジェクト構造

```
ollama-turbo-quant-pe/
├── frontend/                    # non-Turbo フロントエンド
│   ├── src/
│   │   ├── App.tsx            # メインアプリケーション
│   │   ├── components/
│   │   │   ├── PromptEditor.tsx    # プロンプト入力
│   │   │   ├── ResponseViewer.tsx  # レスポンス表示
│   │   │   └── ResponseEditor.tsx  # エディタ（編集機能付き）
│   │   ├── styles.css         # スタイル定義
│   │   └── main.tsx           # エントリーポイント
│   ├── index.html
│   ├── package.json
│   ├── vite.config.ts         # Vite設定（CORSプロキシ）
│   └── tsconfig.json
│
├── backend/                     # non-Turbo バックエンド
│   └── src/
│       ├── main.rs            # サーバー起動
│       ├── handlers.rs        # HTTP/WebSocketハンドラ
│       ├── ollama_client.rs   # Ollama APIクライアント
│       └── types.rs           # リクエスト/レスポンス型
│   └── Cargo.toml
│
├── frontend-turbo/            # Turbo フロントエンド
│   ├── src/
│   │   ├── App.tsx            # enhanceボタン追加版
│   │   ├── components/
│   │   │   ├── PromptEditor.tsx
│   │   │   ├── ResponseViewer.tsx
│   │   │   └── ResponseEditor.tsx
│   │   ├── styles.css
│   │   └── main.tsx
│   ├── index.html
│   ├── package.json
│   ├── vite.config.ts
│   └── tsconfig.json
│
├── backend-turbo/             # Turbo バックエンド
│   └── src/
│       ├── main.rs            # サーバー起動
│       ├── handlers.rs        # HTTP/WebSocket + enhance API
│       ├── ollama_client.rs   # Ollama APIクライアント
│       ├── types.rs           # リクエスト/レスポンス型
│       └── turbo_processor.rs # Turbo-Quant処理
│   └── Cargo.toml
│
├── ready.sh                   # 起動スクリプト
├── launch-non-turbo.sh       # non-Turbo起動（非推奨）
├── launch-turbo.sh           # Turbo起動（非推奨）
├── .gitignore
├── README.md
└── USAGE.md
```

---

## ビルド方法

### 前提条件

- **Rust** 1.75+
- **Node.js** 18+
- **Ollama** インストール済み（localhost:11434で起動）

### Frontend

```bash
# non-Turbo
cd frontend && npm install && npm run build

# Turbo
cd frontend-turbo && npm install && npm run build
```

### Backend

```bash
# non-Turbo
cd backend && cargo build --release

# Turbo
cd backend-turbo && cargo build --release
```

---

## 起動方法

### 一括起動（推奨）

```bash
# non-Turbo Ollama起動
./ready.sh

# Turbo Ollama起動
./ready.sh turbo
```

### 手動起動

```bash
# Terminal 1: non-Turbo
cd backend && cargo run
# Backend: http://localhost:8000

# Terminal 2: non-Turbo Frontend
cd frontend && npm run dev
# Frontend: http://localhost:3000
```

---

## 使い方

### 1. non-Turbo Ollama (http://localhost:3000)

```
┌─────────────────────────────────────────────────────────────┐
│  ┌─────────────────────┐  ┌─────────────────────────────┐  │
│  │     Prompt          │  │        Preview             │  │
│  │  ┌───────────────┐  │  │   LLMレスポンス表示        │  │
│  │  │ Monaco Editor │  │  │   Copy / Accept / Edit     │  │
│  │  └───────────────┘  │  └─────────────────────────────┘  │
│  │  [Model▼] [Clear] │                                    │
│  │  [Generate]        │  ┌─────────────────────────────┐  │
│  └─────────────────────┘  │        Editor              │  │
│                           │   Monaco + ツールバー       │  │
│                           │   Copy / Save / Apply       │  │
│                           └─────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

**手順：**
1. 左上のPrompt Editorに質問を入力
2. モデルを選択（ドロップダウン）
3. 「Generate」ボタンをクリック
4. 右上のPreviewにレスポンスが表示される（ストリーミング）
5. Editor部で編集後「Apply」でPreviewに戻す
6. 「Accept」でPromptに追加

### 2. Turbo Ollama (http://localhost:3001)

```
┌─────────────────────────────────────────────────────────────┐
│  ┌─────────────────────┐  ┌─────────────────────────────┐  │
│  │     Prompt          │  │     Preview (Turbo付き)     │  │
│  │  ~~~~~~~~~~~~~~~~~~ │  │   ~~~~~~~~~~~~~~~~~~~~     │  │
│  │                     │  │   [Enhance (Turbo)] ← NEW │  │
│  │                     │  │   Copy / Accept / Edit     │  │
│  └─────────────────────┘  └─────────────────────────────┘  │
│                           │                                │
│                           │  [Enhance] クリック後:          │
│                           │   ↓                            │
│                           │   Turbo-Quant処理              │
│                           │   ↓                            │
│                           ▼                                │
│                    ┌─────────────────┐                     │
│                    │ Editorに表示   │                     │
│                    │ (変換結果確認) │                     │
│                    └─────────────────┘                     │
└─────────────────────────────────────────────────────────────┘
```

**追加機能：**
- 「Enhance (Turbo)」ボタン: Turbo-Quantでレスポンスを変換

---

## APIエンドポイント

### non-Turbo Backend (port 8000)

| メソッド | エンドポイント | 説明 |
|---------|---------------|------|
| `POST` | `/api/generate` | Ollamaに同期リクエスト |
| `GET` | `/api/models` | Ollamaモデル一覧取得 |
| `WS` | `/ws/generate` | OllamaにWebSocketでストリーミング |

### Turbo Backend (port 8001)

| メソッド | エンドポイント | 説明 |
|---------|---------------|------|
| `POST` | `/api/generate` | Ollamaに同期リクエスト |
| `GET` | `/api/models` | Ollamaモデル一覧取得 |
| `WS` | `/ws/generate` | OllamaにWebSocketでストリーミング |
| `POST` | `/api/enhance` | Turbo-Quantでレスポンス処理 |

### APIリクエスト例

```bash
# モデル一覧取得
curl http://localhost:8000/api/models

# 同期生成
curl -X POST http://localhost:8000/api/generate \
  -H "Content-Type: application/json" \
  -d '{"prompt": "Hello", "model": "qwen3.5:latest", "max_tokens": 50}'

# Turbo enhance
curl -X POST http://localhost:8001/api/enhance \
  -H "Content-Type: application/json" \
  -d '{"text": "Hello, this is a test response."}'
```

---

## Turbo-Quant 処理の詳細

### 処理フロー

```
[テキスト] 
    │
    ▼
┌───────────────────────┐
│ text_to_vector()       │  注1: テキスト→ベクトル変換
│ - 各文字をASCII値に変換│
│ - 単語をハッシュ値に変換│
│ - L2正規化            │
└───────────┬───────────┘
            │
            ▼
┌───────────────────────┐
│ TurboQuantizer.encode()│  注2: ベクトル量子化
│ - ランダム回転        │
│ - PolarQuant (極座標) │
│ - QJLスキザリング     │
└───────────┬───────────┘
            │
            ▼
┌───────────────────────┐
│ decode_approximate()   │  注3: 近似復号
│ - 量子化解除          │
│ - 逆回転              │
└───────────┬───────────┘
            │
            ▼
┌───────────────────────┐
│ ベクトル→文字変換     │  注4: ベクトル→テキスト逆変換
│ - 各値をASCIIに変換   │
└───────────┬───────────┘
            │
            ▼
[変換済みテキスト]
```

### 注釈付きコード（turbo_processor.rs）

```rust
// Turbo-Quant量子化器 생성 → 日本語に修正
// Turbo-Quant量子化器 生成
let turbo = TurboQuantizer::new(
    dimension,  // 次元数（256）
    8,          // ビット数（1-16）
    dimension / 4,  // QJL射影数
    seed        // シード値
)?;

pub fn text_to_vector(&self, text: &str) -> Vec<f32> {
    let mut vector = vec![0.0f32; self.dimension];

    // 注1: 文字コードをベクトル要素に変換
    for (i, c) in text.chars().enumerate() {
        if i >= self.dimension { break; }
        vector[i] = c as u32 as f32 / 255.0;
    }

    // 注2: 単語のハッシュ値も特徴量に追加
    for (i, word) in text.split_whitespace().enumerate() {
        let idx = self.dimension / 2 + (i % (self.dimension / 2));
        let mut hasher = DefaultHasher::new();
        word.hash(&mut hasher);
        vector[idx] = (hasher.finish() as f32 % 1000.0) / 1000.0;
    }

    // 注3: L2正規化
    let magnitude = vector.iter().map(|x| x * x).sum::<f32>().sqrt();
    if magnitude > 0.0 {
        for v in &mut vector { *v /= magnitude; }
    }
    vector
}

pub fn enhance_response(&self, text: &str) -> Result<EnhanceResult> {
    let original_vector = self.text_to_vector(text);

    // 注4: TurboQuantでエンコード（圧縮）
    let code = self.turbo.encode(&original_vector)?;

    // 注5: 近似デコード（復号）
    let approximate = self.turbo.decode_approximate(&code)?;

    // 注6: デコード結果を文字に変換
    let mut enhanced_chars: Vec<char> = Vec::new();
    for (i, &val) in approximate.iter().enumerate() {
        if i < text.len() {
            // 注7: 値をASCII文字范围[32-126]にマッピング
            let scaled = ((val * 255.0).round() as u8).max(32).min(126);
            enhanced_chars.push(scaled as char);
        }
    }

    let enhanced = enhanced_chars.iter().collect::<String>();
    // ...
}
```

### 圧縮率の計算

```
Compression Ratio = 圧縮後サイズ / 圧縮前サイズ

例: dimension=256, bits=8
- 入力: 256 * 4 bytes (f32) = 1024 bytes
- 出力: 256 * 3 bytes + 64 bytes (QJL) = 832 bytes
- Ratio: 832 / 1024 = 1.23x
```

---

## 環境変数

| 変数 | デフォルト値 | 説明 |
|------|-------------|------|
| `OLLAMA_URL` | http://localhost:11434 | OllamaサーバーURL |
| `DEFAULT_MODEL` | llama3.2 | デフォルトモデル |
| `BIND_ADDR` | 0.0.0.0:8000 | バインドアドレス |

---

## トラブルシューティング

### 接続エラー
- Ollamaが起動しているか確認: `curl http://localhost:11434/api/tags`
- ポートが競合していないか確認: `lsof -i :3000 -i :8000`

### WebSocketエラー
- ブラウザの開発者ツールConsoleを確認
- Viteプロキシ設定（vite.config.ts）を確認

### Turbo-Quant処理後のテキストが崩れる
- これは正常動作です
- Turbo-Quantは本来ベクトル量子化用途のため、文字列変換はデモ目的です
- 埋め込みモデルとの連携で実用的な用途になります
