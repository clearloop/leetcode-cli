use lc::conf;

fn main() {
    let c = conf::locate();
    c.sync();
    println!("{:#?}", &c);
}
