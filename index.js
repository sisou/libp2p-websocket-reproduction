const { w3cwebsocket } = require('websocket');
const { connect } = require("./pkg/libp2p_websocket_reproduction.js");

global.WebSocket = w3cwebsocket;
global.WorkerGlobalScope = global;

// process.on('uncaughtException', error => {
//     if (error.message.includes('closure invoked recursively')) {
//         console.log(error.message);
//         return;
//     }
//     throw error;
// });

async function run(addr) {
    console.log("Connecting...");
    await connect(addr);
    console.log("Done");
}

async function connect_seed() {
    await run("/dns4/seed1.pos.nimiq-testnet.com/tcp/8443/wss")
}
connect_seed();
