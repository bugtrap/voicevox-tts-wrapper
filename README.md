# VOICEVOX TTS Wrapper

VOICEVOX の REST API を OpenAI TTS 互換 API として公開するプロキシです。  
OpenAI TTS API を利用するクライアントから、そのまま VOICEVOX の音声合成を利用できます。

## アーキテクチャ

```
Client
  │
  │ HTTPS (x-api-key ヘッダー)
  ▼
API Gateway REST API (v1)
  │  API Key + Usage Plan 認証
  │  アクセスログ / 実行ログ / X-Ray トレース
  ▼
Lambda (Rust, arm64, provided.al2023)
  │
  │ HTTP (reqwest)
  ▼
VOICEVOX Engine (外部エンドポイント)
```

## 前提条件

- [Rust](https://www.rust-lang.org/tools/install) (stable)
- [cargo-lambda](https://www.cargo-lambda.info/guide/installation.html)
- [Node.js](https://nodejs.org/) v22+
- [pnpm](https://pnpm.io/)
- [AWS CDK CLI](https://docs.aws.amazon.com/cdk/v2/guide/cli.html) v2
- AWS アカウントおよび認証情報の設定
- 稼働中の VOICEVOX Engine（REST API が到達可能であること）

## セットアップ

```bash
git clone <repository-url>
cd voicevox-tts-wrapper
```

## ビルド

### Lambda (Rust)

```bash
cd voicevox-proxy
cargo lambda build --release --arm64
```

### CDK

```bash
cd cdk
pnpm install
pnpm run build
```

## デプロイ

### 1. VOICEVOX API URL の設定

`cdk/cdk.context.json` に VOICEVOX Engine の URL を設定します。

```json
{
  "voicevoxApiUrl": "https://your-voicevox-endpoint.example.com"
}
```

> `cdk.context.json` は `.gitignore` に含まれています。環境ごとに作成してください。

コマンドラインから設定する場合は `-c` オプションを使用します。

```bash
pnpm cdk deploy -c voicevoxApiUrl=https://your-voicevox-endpoint.example.com
```

### 2. CDK デプロイ

```bash
cd cdk
pnpm run build
pnpm cdk deploy
```

デプロイ完了後、以下が出力されます。

- `ApiUrl` — API Gateway のエンドポイント URL
- `ApiKeyId` — API Key の ID

### 3. API Key の取得

```bash
aws apigateway get-api-key --api-key <ApiKeyId> --include-value --query 'value' --output text
```

## API リファレンス

すべてのエンドポイントで `x-api-key` ヘッダーによる認証が必要です。

### POST /v1/audio/speech

テキストから音声を合成します。

#### リクエスト

```bash
curl -X POST https://<api-url>/prod/v1/audio/speech \
  -H "x-api-key: <your-api-key>" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "zundamon",
    "input": "おはようなのだ。",
    "voice": "normal",
    "response_format": "wav",
    "speed": 1.0
  }' \
  --output speech.wav
```

#### リクエストボディ

| フィールド | 型 | 必須 | デフォルト | 説明 |
|---|---|---|---|---|
| `model` | string | ✅ | — | VOICEVOX キャラクター名 |
| `input` | string | ✅ | — | 合成するテキスト |
| `voice` | string | ✅ | — | ボイススタイル |
| `response_format` | string | — | `wav` | 音声フォーマット（`wav` のみ） |
| `speed` | number | — | `1.0` | 話速（0.25〜4.0） |
| `voicevox_pitch_scale` | number | — | — | ピッチスケール |
| `voicevox_intonation_scale` | number | — | — | 抑揚スケール |
| `voicevox_volume_scale` | number | — | — | 音量スケール |
| `voicevox_output_sampling_rate` | integer | — | — | 出力サンプリングレート |
| `voicevox_output_stereo` | boolean | — | — | ステレオ出力 |

#### レスポンス

- `200 OK` — `Content-Type: audio/wav` で WAV バイナリを返却

### GET /v1/models

利用可能なモデル（キャラクター）の一覧を取得します。

```bash
curl https://<api-url>/prod/v1/models \
  -H "x-api-key: <your-api-key>"
```

#### レスポンス例

```json
{
  "object": "list",
  "data": [
    {
      "id": "zundamon",
      "object": "model",
      "created": 0,
      "owned_by": "voicevox"
    }
  ]
}
```

### エラーレスポンス

```json
{
  "error": {
    "message": "エラーメッセージ",
    "type": "invalid_request_error",
    "code": "invalid_voice"
  }
}
```

| ステータス | 説明 |
|---|---|
| 400 | 不正なリクエスト（バリデーションエラー） |
| 403 | 認証エラー（API Key 不正） |
| 500 | VOICEVOX API エラー / 内部エラー |

## Voice マッピング

`voicevox-proxy/voice-mapping.json` で `model` + `voice` → VOICEVOX `speaker_id` の対応を定義しています。  
このファイルはビルド時にバイナリに埋め込まれます。

### 対応キャラクター

| model | voice（スタイル） |
|---|---|
| `zundamon` | normal, amaama, sexy, tsuntsun, sasayaki, hisohiso, herohero, namidame |
| `shikokumetan` | normal, amaama, sexy, tsuntsun, sasayaki, hisohiso |
| `ankomon` | normal, tsuyotsuyo, yowayowa, kedaruge, sasayaki |
| `kasukabetsumugi` | normal |
| `meimeihimari` | normal |
| `tohokukiritan` | normal |
| `tohokuzunko` | normal |
| `tohokuitako` | normal |

マッピングを追加・変更する場合は `voice-mapping.json` を編集し、再ビルドしてください。

## 設定

### 環境変数（Lambda）

| 変数名 | 説明 |
|---|---|
| `VOICEVOX_API_URL` | VOICEVOX Engine の REST API ベース URL |

### CDK コンテキスト

| キー | 説明 | デフォルト |
|---|---|---|
| `voicevoxApiUrl` | VOICEVOX Engine の URL | `http://localhost:50021` |

## ローカル開発

[cargo-lambda](https://www.cargo-lambda.info/) を使ってローカルでテストできます。

```bash
# VOICEVOX Engine をローカルで起動しておく（デフォルト: http://localhost:50021）

# Lambda エミュレーター起動
cd voicevox-proxy
cargo lambda watch

# 別ターミナルからリクエスト
curl -X POST http://localhost:9000/v1/audio/speech \
  -H "Content-Type: application/json" \
  -d '{
    "model": "zundamon",
    "input": "おはようなのだ。",
    "voice": "normal"
  }' \
  --output speech.wav
```

## プロジェクト構成

```
.
├── voicevox-proxy/          # Rust Lambda 本体
│   ├── src/
│   │   ├── main.rs          # Lambda ハンドラー / ルーティング
│   │   ├── models.rs        # リクエスト / レスポンス型定義
│   │   ├── error.rs         # エラー型
│   │   ├── voice_mapping.rs # voice-mapping.json ローダー
│   │   ├── voicevox_client.rs # VOICEVOX API クライアント
│   │   ├── speech_service.rs  # 音声合成サービス
│   │   └── models_service.rs  # モデル一覧サービス
│   ├── voice-mapping.json   # model+voice → speaker_id マッピング
│   ├── speakers.json        # VOICEVOX 話者情報（参考用）
│   └── Cargo.toml
├── cdk/                     # AWS CDK (TypeScript)
│   ├── bin/cdk.ts           # CDK アプリエントリポイント
│   ├── lib/voicevox-proxy-stack.ts  # スタック定義
│   ├── cdk.json
│   └── cdk.context.json     # 環境固有設定（.gitignore 対象）
└── aidlc-docs/              # AI-DLC 設計ドキュメント
```

## ライセンス

[MIT License](LICENSE)
