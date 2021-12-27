# lila corral

Welcome to the corral

# Config:
```
cat .env
LILA_CORRAL_MONGO_DATABASE=localhost://127.0.0.1
LILA_CORRAL_MONGO_URI=corral
LILA_CORRAL_WEBSERVER_HOST=0.0.0.0
LILA_CORRAL_WEBSERVER_PORT=3000
```

# Run:
```
RUST_LOG=info cargo run
```

# Test:
```
curl -I http://localhost:3000
HTTP/1.1 200 OK
content-type: text/plain; charset=utf-8
content-length: 0
date: Mon, 27 Dec 2021 11:50:05 GMT
```
