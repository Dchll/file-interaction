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
        let data = Vec::<u8>::new();
        Self { name, data }
    }

    fn open(&mut self) -> bool {
        true
    }

    fn close(&mut self) -> bool {
        true
    }

    #[allow(unused)]
    fn read(&mut self) -> ! {
        todo!()
    }
}

fn main() {
    let mut f1 = File::new("f1.txt");
    f1.open();
    f1.read();
    f1.close();
}
