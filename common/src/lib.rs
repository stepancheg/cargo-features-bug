pub fn print_features() {
    if cfg!(feature = "x") {
        println!("feature = \"x\"")
    }
    if cfg!(feature = "y") {
        println!("feature = \"y\"")
    }
}