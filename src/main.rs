use std::path::Path;
use std::process::Command;

// TODO: Take parameters to support arbitrary versions without changing the source
fn stack_path () {
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

    // And then add a few other ones that we one
    let versions = [Path::new("ghc-9.2.5/bin"), Path::new("ghc-8.10.7/bin")];

    let paths = versions.map(|p| stack_root.clone().join(p).to_str().unwrap());

    println!("{}", paths.join(":"));
}

// TODO: Add subcommands for other purposes
fn main() {
    stack_path()
}
