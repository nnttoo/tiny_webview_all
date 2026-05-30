
use std::{
    collections::HashMap,
    pin::Pin,
    sync::Arc, 
};
 
use serde::{Deserialize, Serialize}; 

use crate::{app_ctx::AppMyContextArc, ipc_server_handler::create_ipc_server };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CmdMessage {
    pub cmd: String,
    pub message: String,
}

pub type IpcRoutePin = Pin<Box<dyn Future<Output = CmdMessage> + Send + 'static>>;
type IpcRouteBox = Box<dyn Fn(CmdMessage) -> IpcRoutePin + Send + Sync>; 
type IpcRouteHashMap = HashMap<String, IpcRouteBox>; 
pub type IpcRouteHashMapArc = Arc<IpcRouteHashMap>; 

pub struct IpcRoute {
    pub hashmap: IpcRouteHashMap,
}

impl IpcRoute {
    pub fn new() -> Self {
        IpcRoute {
            hashmap:  HashMap::new(),
        }
    }

    pub fn add_route<F, Fut>(mut self, path: &str, handler: F) ->Self
    where
        F: Fn(CmdMessage) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = CmdMessage> + Send + 'static,
    {
        // let Ok(mut hashmap) = self.hashmap.try_write() else {
        //     return;
        // };
 
        let hashmap = &mut self.hashmap;

        let wrapped_handler = move |req: CmdMessage| {
            let fut = handler(req);
            Box::pin(fut) as IpcRoutePin
        };
        hashmap.insert(path.to_string(), Box::new(wrapped_handler));

        self
    }

    pub fn create_server(self, app_ctx: AppMyContextArc) {

        let hashclone = Arc::new(self.hashmap);
        tokio::spawn(async move {
            create_ipc_server(app_ctx, hashclone).await;
        });
    }
}

