# Block by client IP address

This sample illustrates how you could block clients from accessing a particular resource by maintaining an IP address block list.

The landing page (served at `/`) is a small UI that explains the sample and lets you drive it from the browser: check your access, block your own IP, view the blocklist, and clear it.

Under the hood:

- `GET /check` is the protected resource. It validates the caller's IP against the blocklist and returns `200` when allowed or `401` when blocked.
- The `/admin/blocked-ips` endpoints manage the blocklist (kept in the default key-value store).

## Run the Spin App on your machine

Once you've cloned the repository and moved to the `./samples/block-by-ip` directory, install the dependencies, build and run the app:

```console
spin build
spin up
```

Then open http://localhost:3000/ in a browser and use the buttons, or drive the endpoints directly with `curl` as shown below.

### Accessing the protected route

Send a `GET` request to `/check`, which shows the data if your IP is not blocked:

```console
curl -iX GET http://localhost:3000/check

HTTP/1.1 200 OK
content-type: application/json

{"allowed":true,"ip":"127.0.0.1","message":"If you can read this, you've successfully passed the blocking mechanism."}
```

### Block your own IP

To block your own IP address, send a `POST` request to `/admin/blocked-ips`:

```console
curl -iX POST http://localhost:3000/admin/blocked-ips

HTTP/1.1 200 OK
content-type: application/json

{"ip":"127.0.0.1","message":"Your IP address has been added to the blocklist"}
```

### Try to access the protected route again

Send a `GET` request to `/check` again — this time the request is blocked with a `401`:

```console
curl -iX GET http://localhost:3000/check

HTTP/1.1 401 Unauthorized
content-type: application/json

{"allowed":false,"ip":"127.0.0.1","message":"Sorry, your IP is blocked."}
```

### Retrieve IP blocklist

To get the list of blocked IP addresses, send a `GET` request to `/admin/blocked-ips`:

```console
curl -iX GET http://localhost:3000/admin/blocked-ips

HTTP/1.1 200 OK
content-type: application/json

["127.0.0.1"]
```

### Clear the IP blocklist

To clear the IP blocklist, send a `DELETE` request to `/admin/blocked-ips`:

```console
curl -iX DELETE http://localhost:3000/admin/blocked-ips

HTTP/1.1 204 No Content
```
