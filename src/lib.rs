use libp2p::{
    core::upgrade::Version, identity::Keypair, noise, websocket_websys, yamux, Transport,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn connect(addr: &str) {
    let local_key = Keypair::generate_ed25519();
    let mut transport = websocket_websys::Transport::default()
        .upgrade(Version::V1)
        .authenticate(noise::Config::new(&local_key).unwrap())
        .multiplex(yamux::Config::default())
        .boxed();

    let (_peer_id, conn) = transport
        .dial(addr.parse().unwrap())
        .unwrap()
        .await
        .unwrap();

    drop(conn);
}
