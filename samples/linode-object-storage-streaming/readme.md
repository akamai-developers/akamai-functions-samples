# Streaming files from Linode Object Storage

This is a sample function that streams a file from Linode Object storage and applies a transformation to each chunk.

You need to add the following application variables:

```
export SPIN_VARIABLE_ACCESS_KEY_ID=<access-key-id>
export SPIN_VARIABLE_SECRET_ACCESS_KEY=<secret-access-key>
```

Then, to build and run locally:

```
spin build
spin up
```

Deploying to Akamai Functions:

```
spin aka deploy --variable access_key_id=$SPIN_VARIABLE_ACCESS_KEY_ID --variable secret_access_key=$SPIN_VARIABLE_SECRET_ACCESS_KEY
```


To make requests:

```
curl <your-app-endpoint>/stream/<bucket-name>/<file> --no-buffer
```
