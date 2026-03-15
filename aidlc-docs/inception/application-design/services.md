# Services

## Service Architecture

本アプリケーションはシンプルなリクエスト→変換→プロキシのパターンで構成される。

```
Client (OpenAI TTS 互換)
    |
    v
[Lambda Handler] -- ルーティング
    |
    +---> [Speech Service] -- 音声合成オーケストレーション
    |         |
    |         +---> [Voice Mapping] -- speaker_id 解決
    |         +---> [VOICEVOX Client] -- API 呼び出し
    |
    +---> [Models Service] -- モデル一覧
              |
              +---> [Voice Mapping] -- モデル情報取得
```

## Service Definitions

### Speech Service
- **Role**: 音声合成リクエストのオーケストレーター
- **Orchestration Flow**:
  1. リクエストバリデーション
  2. Voice Mapping で model + voice → speaker_id 解決
  3. VOICEVOX Client で audio_query 生成
  4. 拡張パラメータ（speed, pitch, intonation, volume, output_sampling_rate, output_stereo）を audio_query に適用
  5. VOICEVOX Client で synthesis 実行
  6. WAV バイナリを返却

### Models Service
- **Role**: モデル一覧の提供
- **Orchestration Flow**:
  1. Voice Mapping からモデル名一覧を取得
  2. OpenAI Models API 互換のレスポンスを構築

### VOICEVOX Client
- **Role**: VOICEVOX REST API への HTTP 通信
- **Stateless**: リクエストごとに独立した HTTP 呼び出し
- **Base URL**: 環境変数 `VOICEVOX_API_URL` から取得

### Voice Mapping
- **Role**: voice-mapping.json の管理と検索
- **Lifecycle**: アプリケーション起動時に一度ロード、以降は読み取り専用
- **Data Source**: コンパイル時に `include_str!` で埋め込み
