// This example was adapted from Cloudflare Workers as a familiar starting point for
// demonstrating how you can migrate your workload to a Spin app on Fermyon Wasm Functions.
// Source: https://developers.cloudflare.com/workers/examples/respond-with-another-site/
// The original example is provided by Cloudflare under the MIT License.

import { Hono } from 'hono';
import { fire } from 'hono/service-worker';
import indexHtml from './index.html';
const UPSTREAM = 'https://random-data-api.fermyon.app/animals/json';

const app = new Hono();

// Landing page describing the proxy behavior.
app.get('/', (c) => c.html(indexHtml));

// Dedicated endpoint that responds with the content fetched from another site.
app.get('/site', async () => {
    return fetch(UPSTREAM);
});


fire(app, {fetch: undefined})
