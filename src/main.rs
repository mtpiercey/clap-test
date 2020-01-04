#[macro_use]
extern crate clap;

fn main() {
    use clap::App;

    let yaml_configuration = load_yaml!("cli.yaml");
    let matches = App::from(yaml_configuration).get_matches();

    if matches.is_present("hello") {
        println!("Hello, world!");
    }
}