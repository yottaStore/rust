pub fn get_coords(pointer: &str, prefix: &str) -> Vec<String> {
    let url = pointer.replace(prefix, "");
    let url: Vec<&str> = url.split(':').collect();
    let mut coords: Vec<&str> = url[0].split('.').collect();
    coords[0] = &pointer;
    coords.pop();
    coords.reverse();
    coords.iter().map(|s| s.to_string()).collect()
}
