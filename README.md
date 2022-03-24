# wasm-dev-server
A simple warp based web server that serves the current directory and adds the headers needed for wasm

```
cross-origin-embedder-policy: require-corp
cross-origin-opener-policy: same-origin
```

See https://web.dev/cross-origin-isolation-guide/ for why this is neede