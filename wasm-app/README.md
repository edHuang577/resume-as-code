# wasm-app

This crate contains the rust code for the static web assembly resume application.

The app is compiled using `wasm-pack` with `rollup` for packaging the JavaScript distribution. These commands can be run with:
```
make build
```

After building, the app can be viewed in a web browser by serving this directory. To serve at `localhost:8080` using the python http.server:
```
make serve
```

Alternatively, tools such as the [VSCode plugin Live Server](https://marketplace.visualstudio.com/items?itemName=ritwickdey.LiveServer) can be used to auto-detect changes during development. Using the following setting in your `.vscode/settings.json`:
```json
{
    "liveServer.settings.root": "/wasm-app"
}
```
