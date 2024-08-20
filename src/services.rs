use curl::easy::Easy;

pub trait Checkable {
    fn check_service(&self, curl: &mut Easy) -> Result<String, String>;
}

pub struct WebService {
    service_name: String,
    endpoint_to_check: String,
}

impl WebService {
    pub fn new(service_name: String, endpoint_to_check: String) -> Self {
        WebService {
            service_name,
            endpoint_to_check,
        }
    }
}
impl Checkable for WebService {
    fn check_service(&self, curl: &mut Easy) -> Result<String, String> {
        //println!("Checking {} at {}", self.service_name, self.endpoint_to_check);
        println!("Starting check...");
        println!("Service:  {}", self.service_name);
        println!("Endpoint:  {}", self.endpoint_to_check);

        curl.url(&self.endpoint_to_check).unwrap();
        let curl_result = curl.perform();
        match curl_result {
            Ok(()) => match curl.response_code().unwrap() {
                200 => Ok("Service up and running".to_owned()),
                error_code @ _ => Err(format!("Error with code: {}", error_code).to_owned()),
            },
            Err(error) => {
                return Err(error.to_string().to_owned());
            }
        }
    }
}
