use tendermint_proto::abci::{
    RequestInfo, ResponseInfo, 
    RequestQuery, ResponseQuery, 
    RequestCheckTx, ResponseCheckTx, 
    ResponseCommit
};

use std::collections::HashMap;
use tendermint_abci::Application;

#[derive(Clone)]
struct KeyValueStore {
    _storage: HashMap<String, String>,
}

impl Application for KeyValueStore {
    fn info(&self, _req: RequestInfo) -> ResponseInfo {
        ResponseInfo::default()
    }
    
    fn query(&self, _req: RequestQuery) -> ResponseQuery {
        ResponseQuery::default()
    }

    fn check_tx(&self, _req: RequestCheckTx) -> ResponseCheckTx {
        ResponseCheckTx::default()
    }

    fn commit(&self) -> ResponseCommit {
        ResponseCommit::default()
    }
}

#[tokio::main]
async fn main() {
    let _app = KeyValueStore {
        _storage: HashMap::new(),
    };
   // todo: server implementation
}
