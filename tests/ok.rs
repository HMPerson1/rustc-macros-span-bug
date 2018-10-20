#![feature(proc_macro_hygiene)]
extern crate rustc_macros_span_bug;
use rustc_macros_span_bug::reforest;
#[reforest]
fn foo() {
    let a = 0;
    if true {} else {}
}
