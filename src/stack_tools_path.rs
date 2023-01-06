use std::path::Path;
use std::process::Command;

fn stack_copiler_tools_path(versions: Vec<String>) -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("stack path --compiler-tools-bin")
        .output()
        .unwrap()
        .stdout;

    let stack_root = String::from_utf8(output).unwrap();

    // The Stack compiler tools looks something like
    // ~/.stack/compiler-tools/x86_64-linux-tinfo6/<ghc version>/bin
    // So we drop the `/bin` and `/<ghc-version>` directories
    let stack_root = Path::new(stack_root.trim())
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    // And then generate a few based on the user's input
    versions
        .into_iter()
        .map(|v| format!("ghc-{}/bin", v))
        .map(|p| stack_root.join(Path::new(&p)).to_str().unwrap().to_owned())
        .collect::<Vec<String>>()
        .join(":")
}

fn main() {
    let vs = std::env::args().skip(1).collect::<Vec<String>>();

    println!("{}", stack_copiler_tools_path(vs));
}
