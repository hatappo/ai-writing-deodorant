# ai-deodorant

テキストファイルからAIっぽい書式を削除するCLIツール。

## 機能

- テキストから `**`（太字マーカー）を削除
- `--emoji` フラグで絵文字も削除可能

## インストール

[Releases](https://github.com/fumi/ai-writing-deodorant/releases) からバイナリをダウンロードするか、ソースからビルド：

```bash
cargo install --path .
```

## 使い方

```bash
# ファイルを処理
ai-deodorant input.txt

# 標準入力から読み込み
echo "**太字** テキスト" | ai-deodorant -

# 絵文字も削除
ai-deodorant input.txt --emoji
```

## ライセンス

MIT
