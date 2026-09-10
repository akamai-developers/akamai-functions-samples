// This example was adapted from Cloudflare Workers as a familiar starting point for
// demonstrating how you can migrate your workload to a Spin app on Akamai Functions
// Source: https://developers.cloudflare.com/workers/examples/alter-headers/
// The original example is provided by Cloudflare under the MIT License.

import { Context, Hono } from 'hono';
import * as variables from '@spinframework/spin-variables';
import { fire } from 'hono/service-worker';
import { logger } from 'hono/logger';

// A fixed upstream path used by the /headers demo endpoint. The origin host is
// configurable via the `origin_host` Spin variable (see spin.toml).
const ORIGIN_PATH = '/animals/json';

const app = new Hono();

app.use(logger());
// Proxy a response from the origin and alter its headers on the way back.
app.get('/headers', async (c: Context) => {
  const originHost = variables.get('origin_host');
  if (!originHost) {
    return c.text('Origin site not configured', 500);
  }

  const requestUrl = new URL(c.req.url);
  // Point the request at the origin. Protocol/port are normalized so local
  // testing (http, port 3000) still reaches the upstream over https.
  requestUrl.protocol = 'https:';
  requestUrl.port = '';
  requestUrl.host = originHost;
  requestUrl.pathname = ORIGIN_PATH;

  const originResponse = await fetch(requestUrl.toString());

  // Wrap the origin response so its headers become mutable.
  const response = new Response(originResponse.body, originResponse);

  // Add a header
  response.headers.append('friendly-message', 'Hello from Akamai Functions!');

  // Delete a header
  response.headers.delete('content-type');

  // Modify a header
  response.headers.set('date', 'the eleventy-sixth of June');

  return response;
});

app.get("*", (c: Context) => {
  c.status(404);
  return c.text("Not Found")
});

fire(app, { fetch: undefined });
