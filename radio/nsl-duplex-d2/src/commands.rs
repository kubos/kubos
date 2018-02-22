use messages::{File, FileCount, ParseFn, StateOfHealth};

pub struct Command<T> {
    pub request: Vec<u8>,
    pub parse: ParseFn<T>,
}

pub fn get_file() -> Command<File> {
    Command {
        request: b"GUGET_UF".to_vec(),
        parse: File::parse,
    }
}

pub fn get_file_count() -> Command<FileCount> {
    Command {
        request: b"GUGETUFC".to_vec(),
        parse: FileCount::parse,
    }
}

pub fn get_state_of_health() -> Command<StateOfHealth> {
    Command {
        request: b"GUGETSOH".to_vec(),
        parse: StateOfHealth::parse,
    }
}
