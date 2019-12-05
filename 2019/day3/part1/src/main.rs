fn main() {
    let bytes = include_bytes!("input.txt");
    let input = String::from_utf8_lossy(bytes);
    let wire_paths = input.split(',').collect::<Vec<_>>();

    println!("{}", wire_paths.join(" - "));
}
