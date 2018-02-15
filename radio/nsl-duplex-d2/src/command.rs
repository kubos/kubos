// TODO: pull this up as 'Command' to the Radio API
pub trait Command<T> {
    fn command(&self) -> u64;
    fn process_response(&self, &[u8]) -> Result<T, String>;
}
