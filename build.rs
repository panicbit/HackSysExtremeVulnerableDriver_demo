use std::env;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    compile("src/shellcode.s", out_dir + "/shellcode");
}

fn compile(input: &str, output: impl AsRef<str>) {
    Command::new("nasm")
        .args(&["-o", output.as_ref()])
        .arg(input)
        .status()
        .unwrap();
}
