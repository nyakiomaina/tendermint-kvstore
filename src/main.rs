use std::sync::Mutex;
use std::fs;

use tendermint_abci::ServerBuilder;
use tendermint_proto::abci::{
    RequestInfo, ResponseInfo, 
    RequestQuery, ResponseQuery, 
    RequestCheckTx, ResponseCheckTx, 
    ResponseCommit,
};

use std::collections::HashMap;
use tendermint_abci::Application;
// use tendermint_proto::v0_34::proto::v0_37::abci::RequestDeliverTx;
#[derive(Clone)]
struct KeyValueStore {
    storage: HashMap<String, String>,
}

impl Application for KeyValueStore {
    fn info(&self, _req: RequestInfo) -> ResponseInfo {
        ResponseInfo::default()
    }
    
    fn query(&self, req: RequestQuery) -> ResponseQuery {
        let key = String::from_utf8_lossy(&req.data).to_string();
        match self.storage.get(&key) {
            Some(value) => {
                let mut response = ResponseQuery::default();
                response.value = value.clone().into_bytes().into();
                response
            }
            None => ResponseQuery::default(),
        }
    }

    fn check_tx(&self, req: RequestCheckTx) -> ResponseCheckTx {
        if req.tx.is_empty() {
            let mut response = ResponseCheckTx::default();
            response.code = 1;
            response.log = "Empty TX".into();
            response
        } else {
            ResponseCheckTx::default()
        }
    }

    // fn deliver_tx(&self, req: RequestDeliverTx) -> RequestDeliverTx {
    //     let parts: Vec<&[u8]> = req.tx.split(|b| *b == b'=').collect();
    //     if parts.len() == 2 {
    //         let key = String::from_utf8_lossy(parts[0]);
    //         let value = String::from_utf8_lossy(parts[1]);
    //         self.storage.insert(key.to_string(), value.to_string());
    //     }
    //     ResponseDeliverTx::default()
    // }

    fn commit(&self) -> ResponseCommit {
        let storage = self.storage.lock().unwrap();
        let serialized = serde_json::to_string(&*storage).expect("Failed to serialize data");
            fs::write("app_state.json", serialized).expect("Unable to write to file");
        ResponseCommit::default()   
    }
}

#[tokio::main]
async fn main() {
    let initial_state = match fs::read_to_string("app_state.json") {
        Ok(data) => serde_json::from_str(&data).expect("Failed to deserialize data"),
        Err(_) => HashMap::new(),
    };

    let app = KeyValueStore {
        storage: HashMap::new(),
    };
   // server implementation
   let server_builder = ServerBuilder::default();
   let server = server_builder.bind("127.0.0.1:26658", app).expect("Failed to binf server to address");

   server.listen().expect("Failed to listen for connections");
}
