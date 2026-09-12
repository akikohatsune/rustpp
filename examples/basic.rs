use rustpp::constants::*;
use rustpp::ezpp::Ezpp;

fn main() {
    let mut ez = Ezpp::new();
    ez.set_mods(MODS_HD | MODS_DT);
    ez.set_accuracy_percent(98.0);
    ez.set_combo(400);
    ez.set_nmiss(1);

    let res = ez.parse_file("tests/fixtures/sample.osu");
    if let Err(e) = res {
        eprintln!("Error calculating: {}", errstr(e));
        return;
    }

    println!("{} - {} [{}]", ez.artist(), ez.title(), ez.version());
    println!("{:.2} stars", ez.stars());
    println!("{:.2} pp", ez.pp());
}
