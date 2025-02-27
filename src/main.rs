#![allow(unused_variables)]
#![allow(unreachable_code)]

type File = String;

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(unused)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    todo!()
}

fn main() {
    let mut f1 = File::from("f1.txt");
    let mut data_f1 = Vec::new();
    open(&mut f1);
    read(&mut f1, &mut data_f1);
    close(&mut f1);
}
