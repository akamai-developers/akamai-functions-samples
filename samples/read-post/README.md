# Read POST body

This sample illustrates how to read the body of an HTTP POST request.

The sample is fronted by a small UI served at `/`, where you can pick a content
type, enter a body, and send a `POST` request to the `/echo` endpoint. The Function
inspects the request based on its `content-type` header and responds with a
description of what it received.

## Try it out

To run the sample, `cd` into the `read-post` directory, then run `spin up --build`.

Open `http://localhost:3000/` in your browser and use the form, or drive the
`/echo` endpoint directly with `curl`:

```
# POST request containing JSON
curl -X POST http://localhost:3000/echo --header "Content-Type: application/json" --data '{"greeting": "hello world"}'
```

```
# POST request containing an HTML form
curl -X POST http://localhost:3000/echo --header "Content-Type: application/x-www-form-urlencoded" --data 'name=ada&subscribe=on'
```

```
# POST request containing an image (you'll need to supply your own)
curl -X POST http://localhost:3000/echo --header "Content-Type: image/jpeg" --data @hobbes.jpg
```

The `/echo` behaviour is also available by posting to the root path (`/`), which
keeps the original sample's `curl http://localhost:3000 --data ...` usage working.
