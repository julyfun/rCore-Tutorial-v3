fn main() {
    // print current dir files
    let paths = std::fs::read_dir(".").unwrap();
    for path in paths {
        println!("{:?}", path.unwrap().path());
    }
}
