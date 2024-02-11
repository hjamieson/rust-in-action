#![allow(unused_variables)]

use rand::random;

#[derive(Debug)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl File {
    fn new(name: &str, data: &Vec<u8>) -> File {
        File {
            name: String::from(name),
            data: data.clone(),
            state: FileState::Closed,
        }
    }

    fn open(&mut self) -> Result<bool, String> {
        self.state = FileState::Open;

        Result::Ok(true)
    }
    fn close(&mut self) -> Result<bool, String> {
        self.state = FileState::Closed;
        Result::Ok(true)
    }
    #[allow(dead_code)]
    fn read(&self, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if random() && random() && random() {
            Result::Err("An error has occured".to_string())
        } else {
            let mut tmp = self.data.clone();
            let read_length = tmp.len();

            save_to.reserve(read_length);
            save_to.append(&mut tmp);
            Result::Ok(read_length)
        }
    }
}

fn main() {
    let mut f2 = File::new("2.txt", &vec![114, 117, 115, 116, 33]);

    let mut buffer: Vec<u8> = vec![];

    f2.open().unwrap();

    let f2_length = f2.read(&mut buffer).unwrap();
    
    f2.close().unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f2);
    println!("{} is {} bytes long", &f2.name, f2_length);
    println!("{}", text);
}

mod other_stuff;
