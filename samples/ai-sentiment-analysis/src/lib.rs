use std::str::FromStr;

use bytes::Bytes;
use spin_sdk::http::body::IncomingBodyExt;
use spin_sdk::http::{FullBody, IntoResponse, Method, Request, Response, StatusCode};
use spin_sdk::http_service;
use spin_sdk::key_value::Store;
use spin_sdk::llm::{InferencingModel::Llama2Chat, InferencingParams, infer_with_options};

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SentimentAnalysisRequest {
    pub sentence: String,
}

#[derive(Serialize)]
pub struct SentimentAnalysisResponse {
    pub sentiment: String,
}

const PROMPT: &str = r#"
<<SYS>>
You are a bot that generates sentiment analysis responses. Respond with a single positive, negative, or neutral.
<</SYS>>
[INST]
Follow the pattern of the following examples:

User: Hi, my name is Bob
Bot: neutral

User: I am so happy today
Bot: positive

User: I am so sad today
Bot: negative
[/INST]

User: {SENTENCE}
"#;

/// A Spin HTTP component that internally routes requests.
#[http_service]
async fn handle_route(req: Request) -> anyhow::Result<impl IntoResponse> {
    if req.method() == Method::POST && req.uri().path() == "/api/sentiment-analysis" {
        perform_sentiment_analysis(req).await
    } else {
        Ok(not_found())
    }
}

fn not_found() -> Response<FullBody<Bytes>> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(FullBody::new(Bytes::from("Not found")))
        .unwrap()
}

async fn perform_sentiment_analysis(req: Request) -> anyhow::Result<Response<FullBody<Bytes>>> {
    let request = body_json_to_map(req).await?;
    // Do some basic clean up on the input
    let sentence = request.sentence.trim();
    println!("Performing sentiment analysis on: {}", sentence);

    // Prepare the KV store
    let kv = Store::open_default()
        .await
        .map_err(|e| anyhow::anyhow!("failed to open key-value store: {e:?}"))?;

    // If the sentiment of the sentence is already in the KV store, return it
    if let Ok(Some(sentiment)) = kv.get(sentence).await {
        println!("Found sentence in KV store returning cached sentiment");
        let resp = SentimentAnalysisResponse {
            sentiment: String::from_utf8(sentiment)?,
        };

        return send_ok_response(StatusCode::OK, resp);
    }
    println!("Sentence not found in KV store");

    // Otherwise, perform sentiment analysis
    println!("Running inference");
    let inferencing_result = infer_with_options(
        Llama2Chat,
        &PROMPT.replace("{SENTENCE}", sentence),
        InferencingParams {
            max_tokens: 6,
            ..Default::default()
        },
    )
    .map_err(|e| anyhow::anyhow!("inference failed: {e:?}"))?;
    println!("Inference result {:?}", inferencing_result);
    let sentiment = inferencing_result
        .text
        .lines()
        .next()
        .unwrap_or_default()
        .strip_prefix("Bot:")
        .unwrap_or_default()
        .parse::<Sentiment>();
    println!("Got sentiment: {sentiment:?}");

    if let Ok(sentiment) = sentiment {
        println!("Caching sentiment in KV store");
        // Cache the result in the KV store
        let _ = kv.set(sentence, sentiment).await;
    }
    let resp = SentimentAnalysisResponse {
        sentiment: sentiment
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
    };

    send_ok_response(StatusCode::OK, resp)
}

fn send_ok_response(
    code: StatusCode,
    resp: SentimentAnalysisResponse,
) -> anyhow::Result<Response<FullBody<Bytes>>> {
    Ok(Response::builder()
        .status(code)
        .header("content-type", "application/json")
        .body(FullBody::new(Bytes::from(serde_json::to_string(&resp)?)))?)
}

async fn body_json_to_map(req: Request) -> anyhow::Result<SentimentAnalysisRequest> {
    let body = req
        .into_body()
        .bytes()
        .await
        .map_err(|e| anyhow::anyhow!("failed to read request body: {e:?}"))?;

    if body.is_empty() {
        anyhow::bail!("Request body was unexpectedly empty");
    }

    Ok(serde_json::from_slice(&body)?)
}

#[derive(Copy, Clone, Debug)]
enum Sentiment {
    Positive,
    Negative,
    Neutral,
}

impl Sentiment {
    fn as_str(&self) -> &str {
        match self {
            Self::Positive => "positive",
            Self::Negative => "negative",
            Self::Neutral => "neutral",
        }
    }
}

impl std::fmt::Display for Sentiment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<[u8]> for Sentiment {
    fn as_ref(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}

impl FromStr for Sentiment {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let sentiment = match s.trim() {
            "positive" => Self::Positive,
            "negative" => Self::Negative,
            "neutral" => Self::Neutral,
            _ => return Err(s.into()),
        };
        Ok(sentiment)
    }
}
