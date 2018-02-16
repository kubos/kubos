// TODO: pull this up as 'Command' to the Radio API
pub trait Command<T> {
    fn command_bytes(&self) -> Vec<u8>;
    fn process_response(&self, &[u8]) -> Result<T, String>;
}
