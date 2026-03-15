# 結合テスト手順

## 概要

Lambda + API Gateway + VOICEVOX API の結合テスト手順。
ローカル環境とデプロイ後の2パターンを記載。

---

## パターン A: ローカルテスト (cargo lambda watch)

### 前提条件
- VOICEVOX ENGINE が起動していること (`http://localhost:50021`)
- cargo-lambda がインストール済み

### 1. ローカル Lambda 起動

```bash
cd voicevox-proxy
VOICEVOX_API_URL=http://localhost:50021 cargo lambda watch
```

Lambda エミュレータが `http://localhost:9000` で起動します。

### 2. 音声合成テスト (POST /v1/audio/speech)

```bash
curl -X POST http://localhost:9000/v1/audio/speech \
  -H "Content-Type: application/json" \
  -d '{
    "model": "zundamon",
    "input": "おはようなのだ。",
    "voice": "normal"
  }' \
  --output test.wav
```

#### 確認項目
- HTTP 200 が返却される
- `test.wav` が有効な WAV ファイルである
- 音声が再生可能である

### 3. モデル一覧テスト (GET /v1/models)

```bash
curl http://localhost:9000/v1/models
```

#### 確認項目
- HTTP 200 + JSON レスポンス
- `object: "list"` を含む
- `data` 配列に `zundamon`, `tohokukiritan` が含まれる

### 4. エラーケーステスト

```bash
# 存在しない model
curl -X POST http://localhost:9000/v1/audio/speech \
  -H "Content-Type: application/json" \
  -d '{"model": "unknown", "input": "test", "voice": "normal"}'

# 存在しない voice
curl -X POST http://localhost:9000/v1/audio/speech \
  -H "Content-Type: application/json" \
  -d '{"model": "zundamon", "input": "test", "voice": "unknown"}'

# 空の input
curl -X POST http://localhost:9000/v1/audio/speech \
  -H "Content-Type: application/json" \
  -d '{"model": "zundamon", "input": "", "voice": "normal"}'

# 不正な response_format
curl -X POST http://localhost:9000/v1/audio/speech \
  -H "Content-Type: application/json" \
  -d '{"model": "zundamon", "input": "test", "voice": "normal", "response_format": "mp3"}'
```

#### 確認項目
- 全て HTTP 400 が返却される
- OpenAI 互換のエラー JSON 形式

---

## パターン B: デプロイ後テスト (API Gateway)

### 前提条件
- CDK デプロイ完了
- API Key 取得済み
- VOICEVOX API がアクセス可能なエンドポイントで稼働

### 1. 音声合成テスト

```bash
API_URL="https://<api-id>.execute-api.<region>.amazonaws.com/prod"
API_KEY="<your-api-key>"

curl -X POST "${API_URL}/v1/audio/speech" \
  -H "Content-Type: application/json" \
  -H "x-api-key: ${API_KEY}" \
  -H "Accept: audio/wav" \
  -d '{
    "model": "zundamon",
    "input": "おはようなのだ。",
    "voice": "normal"
  }' \
  --output test.wav
```

#### 確認項目
- HTTP 200 + WAV バイナリレスポンス
- `test.wav` が再生可能
- `Accept: audio/wav` ヘッダーにより API Gateway がバイナリ変換

### 2. モデル一覧テスト

```bash
curl "${API_URL}/v1/models" \
  -H "x-api-key: ${API_KEY}"
```

### 3. API Key 認証テスト

```bash
# API Key なし → 403
curl -X POST "${API_URL}/v1/audio/speech" \
  -H "Content-Type: application/json" \
  -d '{"model": "zundamon", "input": "test", "voice": "normal"}'

# 不正な API Key → 403
curl -X POST "${API_URL}/v1/audio/speech" \
  -H "Content-Type: application/json" \
  -H "x-api-key: invalid-key" \
  -d '{"model": "zundamon", "input": "test", "voice": "normal"}'
```

#### 確認項目
- API Key なし/不正 → HTTP 403 Forbidden

### 4. speed パラメータテスト

```bash
# 速度 2.0 で合成
curl -X POST "${API_URL}/v1/audio/speech" \
  -H "Content-Type: application/json" \
  -H "x-api-key: ${API_KEY}" \
  -H "Accept: audio/wav" \
  -d '{
    "model": "zundamon",
    "input": "速度テストなのだ。",
    "voice": "normal",
    "speed": 2.0
  }' \
  --output test_fast.wav
```

#### 確認項目
- 通常速度より速い音声が生成される
