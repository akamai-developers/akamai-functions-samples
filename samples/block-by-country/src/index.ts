// For Hono documentation refer to https://hono.dev
import { Hono } from 'hono';
import { fire } from 'hono/service-worker';
import indexHtml from './index.html';
import { evaluateBlock, loadBlocklist, storeBlocklist, lookupCountry } from './blocking';

const app = new Hono();

// Route ordering matters, the first route that matches will be used.
// Any unmatched route will return a 404.

// Serve the demo UI that explains the sample and drives the API below.
app.get('/', (c) => c.html(indexHtml));

// Protected resource: runs the country blocking logic and reports the outcome.
// Returns 200 when allowed and 403 when blocked, always with a JSON body so the
// UI can render either result.
app.get('/check', async (c) => {
  const result = await evaluateBlock(c.req.raw);
  return c.json(result, result.allowed ? 200 : 403);
});

// Admin: list the currently blocked countries.
app.get('/admin/blocked-countries', (c) => c.json(loadBlocklist()));

// Admin: add the caller's own country to the blocklist.
app.post('/admin/blocked-countries', async (c) => {
  const country = await lookupCountry(c.req.raw);
  if (!country) {
    return c.json({ message: 'Could not determine your country from the client IP address.' }, 500);
  }

  const blocklist = loadBlocklist();
  if (blocklist.indexOf(country) > -1) {
    return c.json({ message: `Your country (${country}) is already on the blocklist.`, blocklist });
  }

  blocklist.push(country);
  storeBlocklist(blocklist);
  return c.json({ message: `Your country (${country}) has been added to the blocklist.`, blocklist });
});

// Admin: clear the blocklist.
app.delete('/admin/blocked-countries', (c) => {
  storeBlocklist([]);
  return c.body(null, 204);
});


fire(app, { fetch: undefined})
