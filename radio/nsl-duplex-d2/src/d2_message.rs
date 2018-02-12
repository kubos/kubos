pub trait D2Message {
  fn new(message: Vec<u8>) -> Self;

    fn message(&self) -> & Vec<u8>;

    fn validate(&self) -> Result<(), String> {
       if self.validate_header() && self.validate_body() {
           Ok(())
       } else {
           Err(String::from("Invalid header"))
       }
    }

    // default to true, individual Messages can validate as
    // needed,
    fn validate_body(&self) -> bool {
        true
    }

    fn validate_header(&self) -> bool {
        (self.message()[0] == b'G') && (self.message()[1] == b'U')
    }
}
