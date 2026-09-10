import { Hono } from 'hono';
import { fire } from 'hono/service-worker';
import * as cookie from 'cookie';
import indexHtml from './index.html';

const app = new Hono();

// Landing page with the interactive cookie demo.
app.get('/', (c) => c.html(indexHtml));

// Parse the incoming Cookie header and return the cookies as JSON.
// This is the dedicated endpoint that holds the sample's core logic.
app.get('/cookies', (c) => {
  const cookieHeader = c.req.header('cookie');
  const cookies = cookieHeader ? cookie.parse(cookieHeader) : {};
  return c.json({
    count: Object.keys(cookies).length,
    cookies,
  });
});

fire(app, { fetch: undefined })
