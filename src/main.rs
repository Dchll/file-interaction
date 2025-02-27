#![allow(unused_variables)]
#![allow(unreachable_code)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    fn new(name: &str) -> Self {
        let name = name.to_string();
        // let data = Vec::<u8>::new();
        let data = vec![114, 117, 115, 116, 33];
        Self { name, data }
    }

    fn open(&mut self) -> bool {
        true
    }

    fn close(&mut self) -> bool {
        true
    }

    fn read_to(&self, save_to: &mut Vec<u8>) -> usize {
        let mut tmp = self.data.clone();
        let read_length = self.data.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);

        read_length
    }
}

fn main() {
    let mut f1 = File::new("f1.txt");
    let mut buffer = Vec::<u8>::new();

    f1.open();
    let f1_len = f1.read_to(&mut buffer);
    f1.close();

    let text = String::from_utf8_lossy(&buffer).to_string();

    println!("f1: {:?}", f1);
    println!("text: {} ({} Bytes)", text, f1_len);
}
