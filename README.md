# libp2p-websocket-reproduction

Reproduction for failing Websocket closing in browsers and use-after-free in both browsers and node.

## Instructions

### Web

1. Build for web: `wasm-pack build --target web`
2. Serve root directory, e.g. with `python3 -m http.server`
3. Open the `localhost` site in your browser and open the devtools console
4. Click the "Connect to Seed" button, then wait a few seconds
5. Observe errors in the console when the peer sends data or closes the connection

### NodeJS

1. Build for nodejs: `wasm-pack build --target nodejs`
2. Install node dependencies: `npm i` or equivalent in your favourite npm package manager
3. Start node program: `node index.js`
4. Observe error almost immediately and how node exits the program
