# Cookie Parsing

This sample demonstrates cookie parsing using the NPM `cookie` package. A single
Function serves an interactive UI and a dedicated endpoint that parses the
incoming `Cookie` request header and returns the cookies as JSON.

## Routes

- `GET /` &mdash; browser UI that lets you set a demo cookie and parse it.
- `GET /cookies` &mdash; parses the incoming `Cookie` header and returns the cookies as JSON.

## Try it out

Run `spin up --build`, then open <http://localhost:3000> in a browser. Click
**Set demo cookie**, then **Parse my cookies** to see the parsed result.

You can also call the endpoint directly with `curl`:

```
# One cookie
curl -H "Cookie: AKAMAI_FUNCTIONS_DEMO=hello-from-akamai-functions" localhost:3000/cookies

# Multiple cookies
curl -H "Cookie: AKAMAI_FUNCTIONS_DEMO=hello-from-akamai-functions; token=123" localhost:3000/cookies

# No cookies
curl localhost:3000/cookies
```
