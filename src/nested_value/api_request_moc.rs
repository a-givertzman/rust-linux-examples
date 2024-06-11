///
/// Demo Mok instead real ApiRequest
pub struct ApiRequest {
    value: Vec<u8>,
}
impl ApiRequest {
    pub fn new(value: &[u8]) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
    pub fn fetch(&self) -> Result<Vec<u8>, String> {
        debug!("ApiRequest.fetch | fetched value: {:?}", self.value);
        Ok(self.value.clone())
    }
}