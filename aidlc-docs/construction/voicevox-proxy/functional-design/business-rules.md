# Business Rules

## BR-01: リクエストバリデーション

### BR-01.1: 必須フィールド
- `model`, `input`, `voice` は必須
- いずれかが欠落 → 400 `invalid_request_error` ("Missing required field: {field}")

### BR-01.2: input テキスト
- 空文字列は不可 → 400 `invalid_request_error` ("Input text must not be empty")

### BR-01.3: response_format
- 未指定の場合は "wav" をデフォルト
- "wav" 以外が指定された場合 → 400 `invalid_request_error` ("Unsupported response format: {format}. Only 'wav' is supported")

### BR-01.4: speed パラメータ
- 未指定の場合は 1.0 をデフォルト
- 範囲: 0.25 <= speed <= 4.0
- 範囲外 → 400 `invalid_request_error` ("Speed must be between 0.25 and 4.0")

## BR-02: Voice マッピング解決

### BR-02.1: マッピング検索
1. voice-mapping.json から `model` キーを検索
2. 見つからない場合 → 400 `invalid_request_error` ("Model '{model}' not found")
3. model 内から `voice` キーを検索
4. 見つからない場合 → 400 `invalid_request_error` ("Voice '{voice}' not found for model '{model}'")
5. 見つかった場合 → 対応する speaker_id を返却

### BR-02.2: 数値 voice の扱い
- voice が数値文字列であってもマッピングテーブルに存在しなければエラー
- 数値の直接 speaker_id 変換は行わない

## BR-03: VOICEVOX API 呼び出し

### BR-03.1: audio_query 生成
1. `POST {VOICEVOX_API_URL}/audio_query?text={input}&speaker={speaker_id}`
2. レスポンス: AudioQuery JSON
3. HTTP エラー → 500 `server_error` ("VOICEVOX audio_query failed: {status}")

### BR-03.2: パラメータ適用
AudioQuery JSON に以下のパラメータを上書き:

| リクエストフィールド | AudioQuery フィールド | 変換ルール |
|---|---|---|
| `speed` | `speedScale` | そのまま代入 |
| `voicevox_pitch_scale` | `pitchScale` | 指定時のみ上書き |
| `voicevox_intonation_scale` | `intonationScale` | 指定時のみ上書き |
| `voicevox_volume_scale` | `volumeScale` | 指定時のみ上書き |
| `voicevox_output_sampling_rate` | `outputSamplingRate` | 指定時のみ上書き |
| `voicevox_output_stereo` | `outputStereo` | 指定時のみ上書き |

### BR-03.3: synthesis 実行
1. `POST {VOICEVOX_API_URL}/synthesis?speaker={speaker_id}`
2. Body: 修正済み AudioQuery JSON
3. レスポンス: WAV バイナリ
4. HTTP エラー → 500 `server_error` ("VOICEVOX synthesis failed: {status}")

## BR-04: レスポンス構築

### BR-04.1: 成功レスポンス (POST /v1/audio/speech)
- Status: 200
- Content-Type: `audio/wav`
- Body: WAV バイナリ (Base64 エンコード for API Gateway)

### BR-04.2: モデル一覧レスポンス (GET /v1/models)
- Status: 200
- Content-Type: `application/json`
- Body: voice-mapping.json のキーからモデル一覧を生成

### BR-04.3: エラーレスポンス
- Content-Type: `application/json`
- Body: OpenAI 互換エラー JSON
- Status: AppError variant に応じた HTTP ステータスコード

## BR-05: ルーティング

| Method | Path | Handler |
|---|---|---|
| POST | /v1/audio/speech | Speech Service |
| GET | /v1/models | Models Service |
| * | * | 404 `invalid_request_error` ("Unknown endpoint") |
