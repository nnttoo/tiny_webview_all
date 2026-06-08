use wry::{RequestAsyncResponder, http::{Request, Response}};

// Haryanto 08 June 2026
/// ResponseTools is a struct to handle web responses.
/// This class will be executed in tokio::spawn inside the custom_protocol fn.
pub struct ResponseTools {
    pub req: Request<Vec<u8>>,
}

impl ResponseTools {

    fn create_response(&self, mybyte: &[u8])->Response<Vec<u8>> {
        let response = wry::http::Response::builder()
            .body(mybyte.to_vec())
            .unwrap();

        response
    }

    pub fn read_files(&self) {
        let uri = self.req.uri().to_string();
        println!("halo ini ui nya ya {}", uri);
    }

    pub async fn call_response(&self, res: RequestAsyncResponder) {
        self.read_files(); 

        res.respond(self.create_response(b"ini sekadar test aja ya"));
    }
}
