# Component Dependencies

## Dependency Matrix

| Component | Depends On |
|---|---|
| Lambda Handler | Speech Service, Models Service, Error Types, Models |
| Speech Service | VOICEVOX Client, Voice Mapping, Error Types, Models |
| Models Service | Voice Mapping, Models |
| VOICEVOX Client | Error Types |
| Voice Mapping | Error Types |
| Error Types | Models |
| Models | (none) |

## Communication Patterns

- **Handler → Services**: 直接関数呼び出し（同一プロセス内）
- **Services → VOICEVOX Client**: 直接関数呼び出し（同一プロセス内）
- **VOICEVOX Client → VOICEVOX API**: HTTP (reqwest)
- **Voice Mapping**: コンパイル時埋め込み JSON、起動時パース

## Data Flow

```
Request (JSON)
    |
    v
Handler: parse request body → SpeechRequest
    |
    v
Speech Service: validate → resolve voice → call VOICEVOX
    |
    v
VOICEVOX Client: audio_query(text, speaker_id) → JSON
    |
    v
Speech Service: apply params (speed, pitch, etc.) to query
    |
    v
VOICEVOX Client: synthesis(query, speaker_id) → WAV bytes
    |
    v
Handler: build response (200, audio/wav, WAV bytes)
```

## External Dependencies (Rust Crates)

| Crate | Purpose |
|---|---|
| `lambda_http` | AWS Lambda HTTP イベント処理 |
| `lambda_runtime` | AWS Lambda ランタイム |
| `reqwest` | VOICEVOX API への HTTP クライアント |
| `serde` / `serde_json` | JSON シリアライズ/デシリアライズ |
| `tokio` | 非同期ランタイム |
| `tracing` | 構造化ログ |
