# Domain Entities

## Request / Response Models

### SpeechRequest
OpenAI TTS API 互換のリクエストボディ。

| Field | Type | Required | Description |
|---|---|---|---|
| `model` | String | Yes | モデル名 (e.g., "tts-1") |
| `input` | String | Yes | 読み上げテキスト |
| `voice` | String | Yes | 音声名 (e.g., "alloy") |
| `response_format` | String | No | 音声フォーマット (デフォルト: "wav", "wav" のみサポート) |
| `speed` | f64 | No | 速度 (0.25〜4.0, デフォルト: 1.0) |
| `voicevox_pitch_scale` | f64 | No | ピッチスケール (VOICEVOX 拡張) |
| `voicevox_intonation_scale` | f64 | No | 抑揚スケール (VOICEVOX 拡張) |
| `voicevox_volume_scale` | f64 | No | 音量スケール (VOICEVOX 拡張) |
| `voicevox_output_sampling_rate` | u32 | No | 出力サンプリングレート (VOICEVOX 拡張) |
| `voicevox_output_stereo` | bool | No | ステレオ出力 (VOICEVOX 拡張) |

### ModelsResponse
```json
{
  "object": "list",
  "data": [ModelObject]
}
```

### ModelObject
```json
{
  "id": "tts-1",
  "object": "model",
  "created": 0,
  "owned_by": "voicevox"
}
```

### ErrorResponse
```json
{
  "error": {
    "message": "string",
    "type": "string",
    "code": "string"
  }
}
```

## Voice Mapping Structure

### voice-mapping.json
```json
{
  "<model_name>": {
    "<voice_name>": <speaker_id>
  }
}
```

例:
```json
{
  "tts-1": {
    "alloy": 3,
    "echo": 1,
    "fable": 8,
    "nova": 10,
    "onyx": 13,
    "shimmer": 14
  },
  "tts-1-hd": {
    "alloy": 3,
    "echo": 1,
    "fable": 8,
    "nova": 10,
    "onyx": 13,
    "shimmer": 14
  }
}
```

## VOICEVOX API Types

### AudioQuery (VOICEVOX レスポンス、JSON)
主要フィールド（パラメータ適用対象）:

| Field | Type | Description |
|---|---|---|
| `speedScale` | f64 | 話速スケール |
| `pitchScale` | f64 | ピッチスケール |
| `intonationScale` | f64 | 抑揚スケール |
| `volumeScale` | f64 | 音量スケール |
| `outputSamplingRate` | i32 | 出力サンプリングレート |
| `outputStereo` | bool | ステレオ出力フラグ |

### AppError
```rust
enum AppError {
    Validation { message: String },
    VoiceMapping { message: String },
    VoicevoxApi { message: String, status: u16 },
    Internal { message: String },
}
```

| Variant | HTTP Status | error.type | 発生条件 |
|---|---|---|---|
| Validation | 400 | invalid_request_error | 入力バリデーション失敗 |
| VoiceMapping | 400 | invalid_request_error | model/voice マッピング不在 |
| VoicevoxApi | 500 | server_error | VOICEVOX API エラー |
| Internal | 500 | server_error | 内部エラー |
