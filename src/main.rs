#![cfg(not(test))]

extern crate m;
extern crate rustbox;

use m::{Editor, Input};
use rustbox::{InitOptions, InputMode, OutputMode, RustBox};
use std::env;
use std::io::stdin;

fn main() {
    let source = if env::args().len() > 1 {
        Input::Filename(env::args().nth(1))
    } else {
        Input::Stdin(stdin())
    };

    let rb = match RustBox::init(InitOptions {
        buffer_stderr: false,
        input_mode: InputMode::Esc,
        output_mode: OutputMode::EightBit,
    }) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e),
    };

    let mut editor = Editor::new(source, rb);
    editor.start();
}
