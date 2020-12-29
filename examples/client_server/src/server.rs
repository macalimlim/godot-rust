use gdnative::prelude::*;
use gdnative::api::NetworkedMultiplayerENet;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct Server {
    port: i64,
    max_clients: i64,
    in_bandwidth: i64,
    out_bandwidth: i64,
}

#[methods]
impl Server {
    fn new(_owner: &Node) -> Self {
        Self {
            port: 9876,
            max_clients: 1,
            in_bandwidth: 1000,
            out_bandwidth: 1000,
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Node) {
        let peer = NetworkedMultiplayerENet::new();
        peer.create_server(
            self.port,
            self.max_clients,
            self.in_bandwidth,
            self.out_bandwidth,
        ).unwrap();

        if let Some(tree) = owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            tree.set_network_peer(peer);
        };
    }

    #[export(rpc = "master")]
    fn greet_server(&mut self, owner: &Node, msg: GodotString) {
        godot_print!("Client says: {}", msg);
        owner.rpc(GodotString::from_str("return_greeting"), &[Variant::from_str("hello")]);
    }
}

