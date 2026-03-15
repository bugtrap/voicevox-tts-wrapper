# Business Logic Model

## Speech Synthesis Flow

```
POST /v1/audio/speech
    |
    v
[1. Parse JSON body → SpeechRequest]
    |
    v
[2. Validate request (BR-01)]
    | - model, input, voice 必須チェック
    | - input 空文字チェック
    | - response_format チェック (wav のみ)
    | - speed 範囲チェック (0.25〜4.0)
    |
    v
[3. Resolve speaker_id (BR-02)]
    | - voice-mapping.json[model][voice] → speaker_id
    | - 未発見 → エラー
    |
    v
[4. Call VOICEVOX audio_query (BR-03.1)]
    | - POST /audio_query?text={input}&speaker={speaker_id}
    | - → AudioQuery JSON
    |
    v
[5. Apply parameters (BR-03.2)]
    | - speed → speedScale
    | - voicevox_pitch_scale → pitchScale (optional)
    | - voicevox_intonation_scale → intonationScale (optional)
    | - voicevox_volume_scale → volumeScale (optional)
    |
    v
[6. Call VOICEVOX synthesis (BR-03.3)]
    | - POST /synthesis?speaker={speaker_id}
    | - Body: modified AudioQuery
    | - → WAV binary
    |
    v
[7. Return response (BR-04.1)]
    | - 200, audio/wav, Base64(WAV)
    v
  Done
```

## Models List Flow

```
GET /v1/models
    |
    v
[1. Load voice-mapping.json models]
    | - voice-mapping.json のキー一覧取得
    |
    v
[2. Build ModelsResponse (BR-04.2)]
    | - 各モデル名 → ModelObject
    | - object: "model", owned_by: "voicevox"
    |
    v
[3. Return JSON response]
    | - 200, application/json
    v
  Done
```

## Error Handling Flow

```
Any error during processing
    |
    v
[1. Map to AppError variant]
    | - Parse error → Validation
    | - Missing field → Validation
    | - Voice not found → VoiceMapping
    | - VOICEVOX HTTP error → VoicevoxApi
    | - Unexpected → Internal
    |
    v
[2. Convert to ErrorResponse (BR-04.3)]
    | - message, type, code
    |
    v
[3. Return JSON error with status code]
    v
  Done
```

## Lambda Initialization

```
Lambda cold start
    |
    v
[1. Load voice-mapping.json]
    | - include_str! (compile-time embed)
    | - Parse JSON → HashMap
    |
    v
[2. Read VOICEVOX_API_URL env var]
    |
    v
[3. Initialize VoicevoxClient]
    | - reqwest::Client + base_url
    |
    v
[4. Ready to handle requests]
```
