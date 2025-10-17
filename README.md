# json2kv

標準入力からJSONを受け取り、KeyValue形式のファイルに変換するRustプログラムです。

## 概要

このツールは、JSON形式のデータを読み取り、フラットなKeyValue形式に変換します。ネストされたJSON構造は、キーをドット(`.`)で連結して表現します。

## KeyValue形式の仕様

- 1つのJSON key-valueペアを1行に格納
- keyとvalueは空白（スペース）で区切る
- keyとvalueの識別は最初に登場する空白で区切る
- keyに空白が含まれる場合は`-`（ハイフン）に変換
- valueは空白をそのまま含むことができる
- keyにもvalueにも改行を含めない
  - keyに改行（`\n`、`\r`、`\r\n`）が含まれる場合はエラー終了
  - valueに改行が含まれる場合は以下のように処理：
    - `\r\n`（Windows形式）は事前に`\n`に正規化
    - `\r`（古いMac形式）も事前に`\n`に正規化
    - 正規化された`\n`を`\n`にエスケープ
- ヌル文字など危険な文字が含まれる場合はエラー終了
- ネストされたJSON構造は、キーを`.`で連結

## インストール

```bash
# ビルド
cargo build --release

# バイナリは target/release/json2kv に生成されます
```

## 使用方法

```bash
# 標準入力からJSONを受け取る
echo '{"name": "Alice", "age": 30}' | ./target/release/json2kv

# ファイルから読み込む
cat input.json | ./target/release/json2kv

# 出力ファイルを指定
cat input.json | ./target/release/json2kv > output.kv
```

## 入出力例

### 例1: シンプルなJSON

**入力:**
```json
{
  "name": "Alice",
  "age": 30,
  "city": "Tokyo"
}
```

**出力:**
```
age 30
city Tokyo
name Alice
```

### 例2: ネストされたJSON

**入力:**
```json
{
  "user": {
    "name": "Bob",
    "address": {
      "city": "Osaka",
      "zip": "530-0001"
    }
  },
  "active": true
}
```

**出力:**
```
active true
user.address.city Osaka
user.address.zip 530-0001
user.name Bob
```

### 例3: keyに空白が含まれる場合

**入力:**
```json
{
  "user name": "Charlie",
  "email address": "charlie@example.com"
}
```

**出力:**
```
email-address charlie@example.com
user-name Charlie
```

### 例4: valueに改行が含まれる場合

**入力:**
```json
{
  "description": "This is\na multi-line\ntext"
}
```

**出力:**
```
description This is\na multi-line\ntext
```

**注意:** 改行形式（`\r\n`、`\r`、`\n`）に関わらず、すべて`\n`に正規化されてからエスケープされます。これにより、Windows、Mac、Linuxなど異なるプラットフォームの改行形式を統一的に扱えます。

### 例5: 配列を含むJSON

**入力:**
```json
{
  "tags": ["go", "json", "cli"],
  "count": 3
}
```

**出力:**
```
count 3
tags.0 go
tags.1 json
tags.2 cli
```

## エラー処理

以下の場合はエラー終了します：

- 不正なJSON形式
- keyに改行（`\n`、`\r`、`\r\n`）が含まれる場合
- ヌル文字（`\0`）が含まれる場合
- その他の制御文字が不適切に含まれる場合

**注意:** valueに改行が含まれる場合はエラーにならず、自動的にエスケープされます。

## ライセンス

MIT License
