# Requirements Verification Questions

VOICEVOX API → OpenAI TTS 互換プロキシ (Rust Lambda) の要件を明確にするため、以下の質問にお答えください。

各質問の `[Answer]:` タグの後に、選択肢の文字を記入してください。

## Question 1
OpenAI TTS API の互換範囲はどこまでを想定していますか？

A) `/v1/audio/speech` エンドポイントのみ（テキスト→音声変換の基本機能）
B) `/v1/audio/speech` + モデル一覧 (`/v1/models`) のエンドポイント
C) `/v1/audio/speech` + `/v1/models` + `/v1/audio/transcriptions`（文字起こし）も含む
D) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 2
OpenAI TTS API の `voice` パラメータと VOICEVOX の話者（speaker）のマッピングはどのように行いますか？

A) OpenAI の voice 名（alloy, echo, fable 等）を VOICEVOX の speaker_id に固定マッピングする
B) voice パラメータに VOICEVOX の speaker_id を直接数値で渡す
C) マッピングテーブルを環境変数や設定ファイルで外部管理する
D) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 3
レスポンスの音声フォーマットはどうしますか？OpenAI TTS API は `mp3`, `opus`, `aac`, `flac`, `wav`, `pcm` をサポートしています。VOICEVOX はデフォルトで WAV を返します。

A) WAV のみ返す（フォーマット変換なし、シンプル実装）
B) WAV をデフォルトとし、リクエストで指定された場合は mp3 等に変換する
C) OpenAI API と同じフォーマットオプションをすべてサポートする
D) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 4
Lambda のデプロイ方法・IaC はどうしますか？

A) AWS SAM (Serverless Application Model) を使用
B) AWS CDK (Cloud Development Kit) を使用
C) Terraform を使用
D) デプロイ定義は不要（コードのみ生成）
E) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 5
API Gateway の認証方式はどうしますか？

A) 認証なし（オープンアクセス）
B) API Key による認証（OpenAI API と同様に `Authorization: Bearer <key>` ヘッダー）
C) IAM 認証
D) Cognito ユーザープール認証
E) Other (please describe after [Answer]: tag below)

[Answer]: B

## Question 6
VOICEVOX API の速度・ピッチ等のパラメータ調整は必要ですか？OpenAI TTS API には `speed` パラメータ（0.25〜4.0）があります。

A) OpenAI の `speed` パラメータを VOICEVOX の `speedScale` にマッピングする
B) VOICEVOX のデフォルト設定のみ使用（パラメータ調整なし）
C) speed に加え、追加パラメータ（ピッチ等）もカスタムヘッダーやボディで受け付ける
D) Other (please describe after [Answer]: tag below)

[Answer]: C

## Question 7
エラーハンドリングのレベルはどうしますか？

A) 基本的なエラーハンドリング（VOICEVOX API エラーを OpenAI 互換のエラーレスポンスに変換）
B) 詳細なエラーハンドリング（リトライ、タイムアウト、回路遮断パターン含む）
C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question 8
ストリーミングレスポンスは必要ですか？OpenAI TTS API はストリーミングをサポートしています。

A) ストリーミング不要（VOICEVOX の応答を一括で返す）
B) ストリーミング対応が必要
C) Other (please describe after [Answer]: tag below)

[Answer]: A

## Question: Security Extensions
このプロジェクトにセキュリティ拡張ルールを適用しますか？

A) Yes — すべての SECURITY ルールをブロッキング制約として適用する（本番グレードのアプリケーション向け推奨）
B) No — すべての SECURITY ルールをスキップする（PoC、プロトタイプ、実験的プロジェクト向け）
X) Other (please describe after [Answer]: tag below)

[Answer]: B

