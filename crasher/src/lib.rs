#![feature(proc_macro_hygiene)]
extern crate parse_emit;
use parse_emit::my_proc_macro;
#[my_proc_macro]
fn crashy() {
    if true {} else {}
    macro_rules! m {()=>()}
}
