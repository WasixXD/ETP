#[derive(Debug, Clone)]
pub struct Response {
    pub version: Vec<u8>,
    pub method: Vec<u8>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn sucess(sucess_body: Vec<u8>) -> Self {
        Self {
            version: "ETP/1.0".as_bytes().to_vec(),
            method: "👍".as_bytes().to_vec(),
            body: sucess_body,
        }
    }

    


    pub fn smash(&self) -> Vec<u8> {
        [self.version.clone(), self.method.clone(), self.body.clone()].concat()
    }
}
