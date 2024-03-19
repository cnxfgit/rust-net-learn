
use std::{env, fs::File, io::Write, path::Path};
extern crate cc;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("hello.rs");
    let mut f = File::create(&dest_path).unwrap();

    f.write_all(b"
        fn say_hello() -> &'static str{
            \"hello\"
        }
    ").unwrap();

    cc::Build::new().file("src/hello.c").compile("hello");
}
