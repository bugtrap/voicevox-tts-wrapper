# Component Methods

## 1. Lambda Handler

```rust
// エントリーポイント: API Gateway リクエストをルーティング
async fn handler(event: ApiGatewayV2httpRequest) -> ApiGatewayV2httpResponse

// POST /v1/audio/speech → speech_service::synthesize
// GET /v1/models → models_service::list_models
// その他 → 404 エラー
```

## 2. Speech Service

```rust
// 音声合成メイン処理
async fn synthesize(
    request: SpeechRequest,
    voicevox_client: &VoicevoxClient,
    voice_mapping: &VoiceMapping,
) -> Result<Vec<u8>, AppError>

// リクエストバリデーション
fn validate_request(request: &SpeechRequest) -> Result<(), AppError>
```

## 3. Models Service

```rust
// モデル一覧取得
fn list_models(voice_mapping: &VoiceMapping) -> ModelsResponse
```

## 4. VOICEVOX Client

```rust
// クライアント初期化（環境変数から URL 取得）
fn new(base_url: &str) -> Self

// 音声合成クエリ生成
async fn audio_query(&self, text: &str, speaker_id: u32) -> Result<serde_json::Value, AppError>

// 音声合成実行
async fn synthesis(&self, query: &serde_json::Value, speaker_id: u32) -> Result<Vec<u8>, AppError>

// 話者一覧取得
async fn speakers(&self) -> Result<serde_json::Value, AppError>
```

## 5. Voice Mapping

```rust
// マッピング初期化（埋め込み JSON から）
fn load() -> Result<Self, AppError>

// model + voice → speaker_id 解決
fn resolve(&self, model: &str, voice: &str) -> Result<u32, AppError>

// モデル名一覧取得
fn models(&self) -> Vec<String>

// 指定モデルの voice 一覧取得
fn voices(&self, model: &str) -> Vec<String>
```

## 6. Error Types

```rust
// アプリケーションエラー列挙型
enum AppError {
    Validation { message: String },
    VoiceMapping { message: String },
    VoicevoxApi { message: String, status: u16 },
    Internal { message: String },
}

// AppError → OpenAI 互換エラーレスポンス変換
impl AppError {
    fn to_response(&self) -> ApiGatewayV2httpResponse
    fn error_type(&self) -> &str
    fn status_code(&self) -> u16
}
```
