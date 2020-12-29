use gdnative::prelude::*;
use gdnative::api::NetworkedMultiplayerENet;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ServerPuppet {
    address: String,
    port: i64,
    in_bandwidth: i64,
    out_bandwidth: i64,
    client_port: i64,
}

#[methods]
impl ServerPuppet {
    fn new(_owner: &Node) -> Self {
        Self {
            address: String::from("127.0.0.1"),
            port: 9876,
            in_bandwidth: 1000,
            out_bandwidth: 1000,
            client_port: 9877,
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node>) {
        let peer = NetworkedMultiplayerENet::new();
        peer.create_client(
            GodotString::from(&self.address),
            self.port,
            self.in_bandwidth,
            self.out_bandwidth,
            self.client_port,
        ).unwrap();

        if let Some(tree) = owner.get_tree() {
            let tree = unsafe { tree.assume_safe() };
            tree.set_network_peer(peer);
            tree.connect(
                "connected_to_server",
                owner,
                "on_connected_to_server",
                VariantArray::new_shared(),
                0
            ).unwrap();
        };
    }

    #[export]
    fn on_connected_to_server(&mut self, owner: TRef<Node>) {
        owner.rpc_id(1, GodotString::from_str("greet_server"), &[Variant::from_str("hello")]);
    }

    #[export(rpc = "puppet")]
    fn return_greeting(&mut self, _owner: &Node, msg: GodotString) {
        godot_print!("Server says: {}", msg);
    }
}
