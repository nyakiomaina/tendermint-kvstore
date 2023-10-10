use std::sync::Mutex;
use std::fs;
use serde::{Serialize, Deserialize};

use tendermint_abci::{ServerBuilder, Application};
use tendermint_proto::abci::{
    RequestInfo, ResponseInfo, 
    RequestQuery, ResponseQuery, 
    RequestCheckTx, ResponseCheckTx, 
    ResponseCommit,
};
use tendermint_proto::v0_37::abci::{RequestDeliverTx, ResponseDeliverTx};
use std::collections::HashMap;
#[derive(Debug, Serialize, Deserialize)]
struct KeyValueStore {
    storage: Mutex<HashMap<String, String>>
}

impl Application for KeyValueStore {
    fn info(&self, _req: RequestInfo) -> ResponseInfo {
        ResponseInfo::default()
    }
    
    fn query(&self, req: RequestQuery) -> ResponseQuery {
        let key = String::from_utf8_lossy(&req.data).to_string();
        match self.storage.lock().unwrap().get(&key) {
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
     fn deliver_tx(&self, req: RequestDeliverTx) -> ResponseDeliverTx {
         let data_str = String::from_utf8_lossy(&req.tx);
         let parts: Vec<&str> = data_str.split('=').collect();

         if parts.len() == 2 {
             let user_id = parts[0].trim().to_string();
             let profile_data = parts[1].trim().to_string();

             let mut storage_guard = self.storage.lock().unwrap();
             storage_guard.insert(user_id, profile_data);
         }

         ResponseDeliverTx::default()
     }

    fn commit(&self) -> ResponseCommit {
        let storage_guard = self.storage.lock().unwrap();
        let serialized = serde_json::to_string(&*storage_guard).expect("Failed to serialize data");
        fs::write("app_state.json", serialized).expect("Unable to write to file");
        
        ResponseCommit::default()   
    }
}

impl Clone for KeyValueStore {
    fn clone(&self) -> Self {
        KeyValueStore {
            storage: Mutex::new(self.storage.lock().unwrap().clone())
        }
    }
}

#[tokio::main]
async fn main() {
    // let initial_state = match fs::read_to_string("app_state.json") {
    //     Ok(data) => serde_json::from_str(&data).expect("Failed to deserialize data"),
    //     Err(_) => HashMap::new(),
    // };

    let app = KeyValueStore {
        storage: Mutex::new(HashMap::new()),
    };
   // server implementation
   let server_builder = ServerBuilder::default();
   let server = server_builder.bind("127.0.0.1:26658", app).expect("Failed to binf server to address");

   server.listen().expect("Failed to listen for connections");
}
