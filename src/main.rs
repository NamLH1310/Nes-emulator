#[macro_use] extern crate bitflags;
mod cpu;


fn main() {
    use crate::cpu::init;
    init();
}
