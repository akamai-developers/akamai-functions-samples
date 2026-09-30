// This example was adapted from Cloudflare Workers as a familiar starting point for
// demonstrating how you can migrate your workload to a Spin app on Akamai Functions.
// Source: https://developers.cloudflare.com/workers/examples/read-post/
// The original example is provided by Cloudflare under the MIT License.

import { Hono } from 'hono';
// @ts-ignore - resolved by the bundler at build time
import indexHtml from './index.html';
import { fire } from 'hono/service-worker';

const app = new Hono();

// Inspect the request body and describe it based on its content type.
async function readRequestBody(request: Request): Promise<string> {
    const contentType = request.headers.get("content-type") || '';
    if (contentType.includes("application/json")) {
        return JSON.stringify(await request.json());
    } else if (contentType.includes("application/text")) {
        return request.text();
    } else if (contentType.includes("text/html")) {
        return request.text();
    } else if (contentType.includes("image/")) {
        let body = (await request.body?.getReader().read())?.value;
        if (body) {
            return `a ${body.length}-byte picture, probably of a cat`;
        } else {
            return "an empty picture";
        }
    } else if (contentType.includes("form")) {
        const formData = await request.formData();
        const body: { [key: string]: any } = {};
        formData.forEach((value, key, _) => {
            body[key] = value;
        });
        return JSON.stringify(body);
    } else {
        return "unrecognised body content";
    }
}

// Serve the UI that lets you compose and send a POST request from the browser.
app.get('/', (c) => c.html(indexHtml));

// Read the POST body and echo back a description of what was received.
// This is the endpoint the UI posts to; it also works directly with curl.
const echo = async (c: any) => {
    const requestInfo = await readRequestBody(c.req.raw);
    return c.text(`The POST body sent was ${requestInfo}`);
};

app.post('/echo', echo);

// Keep the original behaviour available: POST to the root still reads the body.
app.post('/', echo);

fire(app, { fetch: undefined})
