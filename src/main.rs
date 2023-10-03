use tendermint_proto::v0_34::abci::{
    RequestInfo, ResponseInfo, 
    RequestQuery, ResponseQuery, 
    RequestCheckTx, ResponseCheckTx, 
    ResponseCommit
};

use std::collections::HashMap;
use tendermint_abci::Application;


#[derive(Clone)]
struct KeyValueStore {
    storage: HashMap<String, String>,
}

impl Application for KeyValueStore {
    fn info(&self, _req: &RequestInfo) -> ResponseInfo {
        ResponseInfo::new()
    }
    
    fn query(&self, _req: &RequestQuery) -> ResponseQuery {
        ResponseQuery::new()
    }

    fn check_tx(&self, _req: &RequestCheckTx) -> ResponseCheckTx {
        ResponseCheckTx::new()
    }

    fn commit(&self) -> ResponseCommit {
        ResponseCommit::new()
    }

}

#[tokio::main]
async fn main() {
    let app = KeyValueStore {
        storage: HashMap::new(),
    };
   // todo: server implementation
}
