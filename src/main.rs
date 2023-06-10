use std::path::PathBuf;

fn main() {
    simple_logger::init_with_env().unwrap();

    let mp3 = id3v2::read_file(&PathBuf::from("batata.mp3"));

    println!("{:#?}", mp3.unwrap());
}
