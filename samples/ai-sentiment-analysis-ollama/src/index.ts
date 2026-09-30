import { Hono } from 'hono'
import type { Context, Next } from 'hono'
// restoreLocation is required because zod access `location` during global scope evaluation
import { restoreLocation } from './fix'
import * as kv from '@spinframework/spin-kv'
import * as variables from '@spinframework/spin-variables'
import 'abortcontroller-polyfill'
import { Ollama } from '@langchain/ollama'
import { PromptTemplate } from '@langchain/core/prompts'
import { z } from 'zod'

restoreLocation()
let app = new Hono()

const ApiUrlKey = 'apiUrl'
const ModelNameKey = 'modelName'

const ApiUrlVariableName = 'ollama_api_url'
const ModelNameVariableName = 'ollama_model_identifier'

const SentimentAnalysisPromptTemplate = `<<SYS>>
You are a bot that generates sentiment analysis responses. Respond with a single positive, negative, or neutral.
<</SYS>>
[INST]
Follow the pattern of the following examples:

User: Hi, my name is Bob
neutral

User: I am so happy today
positive

User: I am so sad today
negative
[/INST]

User: {sentence}`;

const prompt = new PromptTemplate({
  inputVariables: ['sentence'],
  template: SentimentAnalysisPromptTemplate,
})

interface SentimentAnalysisResponse {
  sentiment: 'neutral' | 'positive' | 'negative'
}

const SentimentAnalysisRequestSchema = z.object({
  sentence: z.string(),
})

app.use(async (c: Context, next: Next) => {
  const ollamaApiUrl = variables.get(ApiUrlVariableName)
  const modelName = variables.get(ModelNameVariableName)
  if (!ollamaApiUrl || !modelName) {
    c.status(500)
    c.text('Application configuration not provided')
    return
  }
  c.set(ApiUrlKey, ollamaApiUrl)
  c.set(ModelNameKey, modelName)
  await next()
});

app.post('/api/sentiment-analysis', async (c: Context) => {
  var payload
  try {
    const json = await c.req.json()
    payload = SentimentAnalysisRequestSchema.parse(json)
  } catch (err) {
    c.status(400)
    return c.text('Invalid Payload received')
  }
  const sentence = payload.sentence.trim()
  const store = kv.openDefault()
  if (store.exists(sentence)) {
    return c.json(store.getJson(sentence))
  }

  const llm = new Ollama({
    baseUrl: c.get(ApiUrlKey),
    model: c.get(ModelNameKey),
    temperature: 0,
    maxRetries: 2,
    numPredict: 6
  })

  const formattedPrompt = await prompt.format({
    sentence: sentence,
  })

  let llmResponse = await llm.invoke(formattedPrompt)
  const sentiment = llmResponse.trim().toLowerCase()
  if (sentiment === 'positive' || sentiment === 'negative' || sentiment === 'neutral') {
    const result = { sentiment } as SentimentAnalysisResponse
    store.setJson(sentence, result)
    return c.json(result)
  }

  c.status(500)
  return c.json({ error: 'Unable to determine sentiment' })
})

//@ts-ignore
addEventListener('fetch', (event: FetchEvent) => {
  event.respondWith(app.fetch(event.request))
})

