use leetcode_cli::cli;
use tokio::runtime::Builder;

fn main() {
    if let Err(err) = Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .expect("Build tokio runtime failed")
        .block_on(cli::main())
    {
        println!("{:?}", err);
    }
}
