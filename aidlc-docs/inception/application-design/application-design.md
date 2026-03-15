# Application Design: VOICEVOX → OpenAI TTS Proxy

## Overview

VOICEVOX REST API を OpenAI TTS 互換 API として公開する Rust Lambda プロキシ。

## Components (7)

1. **Lambda Handler** - API Gateway リクエストのルーティング
2. **Speech Service** - 音声合成オーケストレーション
3. **Models Service** - モデル一覧提供
4. **VOICEVOX Client** - VOICEVOX REST API HTTP クライアント
5. **Voice Mapping** - model + voice → speaker_id マッピング管理
6. **Error Types** - OpenAI 互換エラーレスポンス
7. **Models / DTOs** - リクエスト/レスポンスデータ構造

## Architecture

```
API Gateway (HTTP API + API Key)
    |
    v
Lambda Handler
    |
    +---> POST /v1/audio/speech → Speech Service
    |         |
    |         +---> Voice Mapping (speaker_id 解決)
    |         +---> VOICEVOX Client (audio_query → synthesis)
    |
    +---> GET /v1/models → Models Service
    |         |
    |         +---> Voice Mapping (モデル一覧)
    |
    +---> Other → 404 Error
```

## Key Design Decisions

- **Voice Mapping**: `voice-mapping.json` をコンパイル時に `include_str!` で埋め込み
- **HTTP Client**: `reqwest` で VOICEVOX API と通信
- **Error Handling**: 全エラーを `AppError` enum に集約し、OpenAI 互換 JSON に変換
- **Async**: `tokio` ランタイムで非同期 HTTP 通信
- **Logging**: `tracing` クレートで構造化ログ

## Detailed Design References

- Components: [components.md](components.md)
- Methods: [component-methods.md](component-methods.md)
- Services: [services.md](services.md)
- Dependencies: [component-dependency.md](component-dependency.md)
