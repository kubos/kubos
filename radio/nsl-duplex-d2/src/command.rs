// TODO: pull this up as 'Command' to the Radio API
pub trait Command<T> {
    fn new() -> Self;
    fn command_bytes(&self) -> u64;
    fn process_response(&self, &[u8]) -> Result<T, String>;
}
