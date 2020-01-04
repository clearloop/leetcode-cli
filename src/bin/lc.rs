use leetcode_cli::cli;

fn main() {
    let r = cli::main();
    if r.is_err() {
        println!("{:?}", r.err().expect("This won't happend."));
    }
}
