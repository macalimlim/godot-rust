use gdnative::prelude::*;
use gdnative::api::NetworkedMultiplayerENet;

const ADDRESS: &str = "127.0.0.1";
const PORT: i64 = 9876;
const IN_BANDWIDTH: i64 = 1000;
const OUT_BANDWIDTH: i64 = 1000;
const CLIENT_PORT: i64 = 9877;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct ServerPuppet;

#[methods]
impl ServerPuppet {
    fn new(_owner: &Node) -> Self {
        Self
    }

    #[export]
    fn _ready(&mut self, owner: TRef<Node>) {
        let peer = NetworkedMultiplayerENet::new();
        peer.create_client(
            GodotString::from(ADDRESS),
            PORT,
            IN_BANDWIDTH,
            OUT_BANDWIDTH,
            CLIENT_PORT
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
