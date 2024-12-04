fn main() {
    gui_panic_handler::register();

    println!("Hello, world!");
    panic!("Whaaaaat???");
}
