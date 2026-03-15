# Components

## 1. Lambda Handler (`handler`)
- **Purpose**: API Gateway からのリクエストを受け取り、ルーティングを行うエントリーポイント
- **Responsibilities**:
  - HTTP リクエストのパース（パス、メソッド、ボディ、ヘッダー）
  - エンドポイントルーティング（`/v1/audio/speech`, `/v1/models`）
  - レスポンスの構築（ステータスコード、ヘッダー、ボディ）
  - グローバルエラーハンドリング
- **Interfaces**:
  - Input: `ApiGatewayV2httpRequest`
  - Output: `ApiGatewayV2httpResponse`

## 2. Speech Service (`speech_service`)
- **Purpose**: OpenAI TTS 互換の音声合成リクエストを処理するサービス層
- **Responsibilities**:
  - リクエストボディのバリデーション（model, input, voice, speed 等）
  - Voice マッピング解決（model + voice → speaker_id）
  - VOICEVOX API 呼び出しのオーケストレーション（audio_query → synthesis）
  - 拡張パラメータ（pitch, intonation, volume）の適用
- **Interfaces**:
  - Input: `SpeechRequest`
  - Output: `Vec<u8>` (WAV バイナリ)

## 3. Models Service (`models_service`)
- **Purpose**: OpenAI Models API 互換のモデル一覧を返すサービス層
- **Responsibilities**:
  - voice-mapping.json からモデル一覧を生成
  - OpenAI Models API 互換のレスポンス構築
- **Interfaces**:
  - Input: なし
  - Output: `ModelsResponse`

## 4. VOICEVOX Client (`voicevox_client`)
- **Purpose**: VOICEVOX REST API との HTTP 通信を担当するクライアント
- **Responsibilities**:
  - `POST /audio_query` の呼び出し
  - `POST /synthesis` の呼び出し
  - `GET /speakers` の呼び出し
  - HTTP エラーのハンドリングとラッピング
- **Interfaces**:
  - Input: テキスト、speaker_id、AudioQuery パラメータ
  - Output: AudioQuery JSON、WAV バイナリ、Speakers JSON

## 5. Voice Mapping (`voice_mapping`)
- **Purpose**: model + voice → speaker_id のマッピング管理
- **Responsibilities**:
  - voice-mapping.json の読み込み（コンパイル時埋め込み）
  - model + voice の組み合わせから speaker_id を解決
  - マッピングからモデル一覧・voice 一覧の取得
- **Interfaces**:
  - Input: model 名, voice 名
  - Output: `speaker_id` (u32) or Error

## 6. Error Types (`error`)
- **Purpose**: アプリケーション全体のエラー型定義
- **Responsibilities**:
  - OpenAI 互換エラーレスポンスの構造体定義
  - アプリケーションエラーから HTTP レスポンスへの変換
  - エラー種別: ValidationError, VoiceMappingError, VoicevoxApiError, InternalError
- **Interfaces**:
  - Input: 各種エラー
  - Output: `ApiGatewayV2httpResponse` (JSON エラーボディ)

## 7. Models / DTOs (`models`)
- **Purpose**: リクエスト/レスポンスのデータ構造定義
- **Responsibilities**:
  - `SpeechRequest`: OpenAI TTS リクエストボディ（+ 拡張フィールド）
  - `ModelsResponse` / `ModelObject`: OpenAI Models API レスポンス
  - `ErrorResponse` / `ErrorDetail`: OpenAI 互換エラーレスポンス
  - VOICEVOX 関連の型（AudioQuery 等）
