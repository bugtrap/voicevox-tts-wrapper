# Requirements: VOICEVOX → OpenAI TTS 互換プロキシ

## Intent Analysis

- **User Request**: VOICEVOX の REST API を OpenAI TTS 互換の API にするプロキシを Rust ベースの AWS Lambda で作成する
- **Request Type**: New Project (Greenfield)
- **Scope**: Single Component (1 Lambda + API Gateway)
- **Complexity**: Moderate（API 変換レイヤー、音声パラメータマッピング、CDK デプロイ）

---

## Functional Requirements

### FR-01: OpenAI TTS 互換エンドポイント (`POST /v1/audio/speech`)

- OpenAI TTS API 互換のリクエストボディを受け付ける
- リクエストボディ:
  ```json
  {
    "model": "zundamon",
    "input": "おはようなのだ。",
    "voice": "normal",
    "response_format": "wav",
    "speed": 1.0
  }
  ```
- `input` テキストを VOICEVOX の `audio_query` API に渡し、音声合成クエリを生成
- 生成されたクエリを VOICEVOX の `synthesis` API に渡し、WAV 音声データを取得
- WAV 音声データをレスポンスボディとして返却（`Content-Type: audio/wav`）
- `response_format` は `wav` のみサポート（他フォーマット指定時はエラー返却）

### FR-02: モデル一覧エンドポイント (`GET /v1/models`)

- OpenAI Models API 互換のレスポンスを返す
- VOICEVOX の話者一覧を取得し、モデルとして返却
- レスポンス形式:
  ```json
  {
    "object": "list",
    "data": [
      {
        "id": "zundamon",
        "object": "model",
        "created": 0,
        "owned_by": "voicevox"
      },
      {
        "id": "tohokukiritan",
        "object": "model",
        "created": 0,
        "owned_by": "voicevox"
      }
    ]
  }
  ```

### FR-03: Voice-Speaker マッピング

- OpenAI の `model` + `voice` パラメータの組み合わせで VOICEVOX の `speaker_id` を決定
- マッピングテーブルはバイナリに内蔵する `voice-mapping.json` で管理
  - 形式例:
    ```json
    {
      "zundamon": {
        "normal": 3,
        "amaama": 1,
        "sexy": 5,
        "tsuntsun": 7,
        "sasayaki": 22,
        "hisohiso": 38,
        "herohero": 75,
        "namidame": 76
      },
      "tohokukiritan": {
        "normal": 108
      }
    }
    ```
  - 第1レベルキー: model 名
  - 第2レベルキー: voice 名 → 値: VOICEVOX speaker_id
- マッピングに存在しない model/voice の組み合わせが指定された場合はエラーを返却

### FR-04: 音声パラメータマッピング

- OpenAI の `speed` パラメータ (0.25〜4.0) を VOICEVOX の `speedScale` にマッピング
- 追加パラメータをリクエストボディの拡張フィールドで受け付ける:
  - `voicevox_pitch_scale`: ピッチスケール（VOICEVOX `pitchScale`）
  - `voicevox_intonation_scale`: 抑揚スケール（VOICEVOX `intonationScale`）
  - `voicevox_volume_scale`: 音量スケール（VOICEVOX `volumeScale`）
- 拡張フィールドが未指定の場合は VOICEVOX のデフォルト値を使用

### FR-05: VOICEVOX API 連携

- VOICEVOX REST API エンドポイントは環境変数 `VOICEVOX_API_URL` から取得
- 使用する VOICEVOX API:
  - `POST /audio_query?text={text}&speaker={speaker_id}` - 音声合成クエリ生成
  - `POST /synthesis?speaker={speaker_id}` - 音声合成実行
  - `GET /speakers` - 話者一覧取得

### FR-06: エラーハンドリング

- VOICEVOX API のエラーを OpenAI 互換のエラーレスポンスに変換
- エラーレスポンス形式:
  ```json
  {
    "error": {
      "message": "エラーメッセージ",
      "type": "invalid_request_error",
      "code": "invalid_voice"
    }
  }
  ```
- HTTP ステータスコードマッピング:
  - 400: 不正なリクエスト（バリデーションエラー）
  - 403: 認証エラー（API Gateway が返却）
  - 404: 不明なエンドポイント
  - 500: VOICEVOX API エラー、内部エラー

### FR-07: API Key 認証

- API Gateway REST API (v1) のネイティブ API Key + Usage Plan 機能で認証
- クライアントは `x-api-key` ヘッダーで API Key を送信
- API Key 検証は API Gateway レベルで実施（Lambda に到達する前）
- 不正な API Key の場合は API Gateway が 403 Forbidden を返却

---

## Non-Functional Requirements

### NFR-01: 実装言語・ランタイム
- Rust で実装
- AWS Lambda (provided.al2023 ランタイム) で動作

### NFR-02: デプロイ
- AWS CDK (TypeScript) でインフラ定義
- API Gateway REST API (v1) + Lambda 構成

### NFR-03: 環境変数
- `VOICEVOX_API_URL`: VOICEVOX REST API のベース URL

### NFR-04: レスポンス
- ストリーミング不要（一括レスポンス）
- 音声フォーマットは WAV のみ

### NFR-05: セキュリティ
- 基本的な入力バリデーションは実施

### NFR-06: アクセスログ（必須）
- API Gateway REST API のアクセスログを CloudWatch Logs に記録
- ログフィールド: requestId, ip, httpMethod, requestTime, resourcePath, responseLength, status
- ログ保持期間: 14 日間

### NFR-07: トレース（必須）
- AWS X-Ray によるトレースを有効化
  - API Gateway: ステージレベルで X-Ray トレーシング有効
  - Lambda: Active Tracing モード有効
- API Gateway → Lambda → VOICEVOX API の呼び出しチェーンを可視化

### NFR-08: 実行ログ（必須）
- API Gateway Execution Logging を INFO レベルで有効化
- リクエスト/レスポンスデータのトレース有効（dataTraceEnabled: true）

---

## Technical Decisions

| 項目 | 決定 |
|---|---|
| 言語 | Rust |
| ランタイム | Lambda (provided.al2023) |
| IaC | AWS CDK (TypeScript) |
| API Gateway | REST API (v1) |
| 認証 | API Key (x-api-key ヘッダー) + Usage Plan |
| 音声フォーマット | WAV のみ |
| ストリーミング | なし |
| エラーハンドリング | 基本レベル |
| アクセスログ | 必須（CloudWatch Logs, 14日保持） |
| トレース | 必須（X-Ray: API Gateway + Lambda） |
| 実行ログ | 必須（INFO + データトレース） |
