use messages::{File, FileCount, ParseFn, StateOfHealth};

pub struct Command<T> {
    pub request: Vec<u8>,
    pub parse: ParseFn<T>,
}

pub fn get_file() -> Command<File> {
    Command {
        request: vec![0x47, 0x55, 0x47, 0x45, 0x54, 0x5F, 0x55, 0x46],
        parse: File::parse,
    }
}

pub fn get_file_count() -> Command<FileCount> {
    Command {
        request: vec![0x47, 0x55, 0x47, 0x45, 0x54, 0x55, 0x46, 0x43],
        parse: FileCount::parse,
    }
}

pub fn get_state_of_health() -> Command<StateOfHealth> {
    Command {
        request: vec![0x47, 0x55, 0x47, 0x45, 0x54, 0x53, 0x4F, 0x48],
        parse: StateOfHealth::parse,
    }
}

/*
// TODO: convert to test syntax and add test vectors.
fn request<T>(command: Command<T>) {
    println!("Request {:?}", command.request);
    if let Ok((_, res)) = (command.parse)(b"garbage data") {
        // TODO: figure out how to convince compiler it has Debug
        println!("Result: {:?}", res)
    }
}

fn test() {
    let file = request(get_file());
    let file_count = request(get_file_count());
    let state_of_health = request(get_state_of_health());
}
*/
