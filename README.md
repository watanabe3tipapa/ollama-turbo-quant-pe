![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-green.svg)



# Ollama Turbo Quant

Ollama Turbo Quantは、turbo-quantが提供するベクトル量子化機能（符号化、デコード等）を使って、Ollamaからの出力を後処理できるローカルLLM支援ツールです。

## 概要

Ollama Turbo Quantは、ローカルLLM（Ollama）との対話を支援するWebツールです。turbo-quantライブラリのベクトル量子化機能を活用し、LLMレスポンスの変換・圧縮・品質改善を実現します。

## 2つのモード

| モード | ポート | 説明 |
|--------|--------|------|
| **non-Turbo** | 3000 | Ollamaとの連携のみ |
| **Turbo** | 3001 | Ollama + Turbo-Quantによるレスポンス処理 |

## 主な機能

- **Prompt編集**: Monaco Editorによる高度なテキスト編集
- **ストリーミング生成**: WebSocket経由でのリアルタイムLLM出力
- **モデル選択**: Ollamaに登録されているモデルの一覧表示・選択
- **レスポンス編集**: 生成後の編集・適用機能
- **ファイル保存**: レスポンスのMarkdownファイル出力
- **Turbo-Quant処理**: レスポンスのベクトル量子化による変換（Turboモード）

## 技術スタック

### Frontend
- React 18
- TypeScript 5
- Vite 5
- Monaco Editor

### Backend
- Rust
- Axum 0.7
- Tokio

### Turbo-Quant
- [turbo-quant](https://github.com/RecursiveIntell/turbo-quant)
- nalgebra
- rand

## クイックスタート

```bash
# 前提条件
# - Rust 1.75+
# - Node.js 18+
# - Ollama（localhost:11434で起動済み）

# 起動
./ready.sh        # non-Turbo起動（http://localhost:3000）
./ready.sh turbo  # Turbo起動（http://localhost:3001）
```

## ライセンス

MIT
