mod rpc;

fn main() {
    let rpc = rpc::RPC { 
        bind_ip: "0.0.0.0".to_string(),
        bind_port: "1337".to_string(),
    };

    rpc.listen();
}
