# AI Sentiment Analysis (Ollama Edition)

This sample illustrates how to build an AI Sentiment Analysis using a configurable Large Language Model (LLM) hosted on Ollama.

The Spin App consists of three different components:

- A HTML5 Frontend served using the static fileserver Wasm component for Spin
- A HTTP API implemented using TypeScript and LangChain.js
- The Key-Value Explorer to examine persisted sentiment analysis

The TypeScript API component is built with the modern Spin JS/TS toolchain (`esbuild` + `jco` via `@spinframework/build-tools`, driven by `build.mjs`).

## Configuration Variables

See all available configuration variables in the following table:

| Variable Name | Data Type | Required | Default | Description |
|---------------|-----------|----------|---------|-------------|
| `ollama_api_url` | `string` | `yes` | `` | Root URL of your Ollama Server (e.g., `http://1.1.1.1:11434/`) |
| `ollama_model_identifier` | `string` | `no` | `llama3.2:latest` | Identifier of your LLM running on Ollama |
| `kv_explorer_user` | `string` | `yes` | `` | The username for accessing the Key-Value explorer |
| `kv_explorer_password` | `string` | `yes` | `` | The password for accessing the Key-Value explorer |

## Building the Application

You can build the application by invoking `spin build`.

## Running the Application on your local machine

To run the application, specify the required variables and invoke `spin up`:

```bash
export SPIN_VARIABLE_ollama_api_url=http://0.0.0.0:11434/
export SPIN_VARIABLE_ollama_model_identifier=llama3.2:latest
export SPIN_VARIABLE_kv_explorer_user=bob
export SPIN_VARIABLE_kv_explorer_password=secret

spin up
```

## Using the app

Once the app is running, open [http://localhost:3000](http://localhost:3000) in a
browser, type some text into the box, and click **Analyze sentiment**. The
frontend calls the API component and shows whether the tone is positive,
negative, or neutral.

You can also call the API directly. It exposes `POST /api/sentiment-analysis`
and expects a JSON body with a single `sentence` property:

```bash
curl -X POST http://localhost:3000/api/sentiment-analysis \
  -H "Content-Type: application/json" \
  -d '{"sentence": "I am so happy today"}'
# => {"sentiment":"positive"}
```

Results are cached in the key-value store, so repeating a phrase returns the
cached sentiment without another inference call. You can inspect the cache via
the Key-Value Explorer at `/internal/kv-explorer/`.

## Deploying to Akamai Functions

Once authenticated (`spin aka login`), you can deploy the application using the `spin aka deploy` command. Required variables must be specified using the `--variable` flag:

```bash
spin aka deploy \
  --variable ollama_api_url="http://0.0.0.0:11434/" \
  --variable ollama_model_identifier="llama3.2:latest" \
  --variable kv_explorer_user="bob" \
  --variable kv_explorer_password="secret"
```
