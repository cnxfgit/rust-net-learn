include!(concat!(env!("OUT_DIR"), "/hello.rs"));
extern {fn hello();}

fn main() {
    println!("{}", say_hello());
    unsafe {
        hello();
    }
}