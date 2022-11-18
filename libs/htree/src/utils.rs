pub fn get_coords(pointer: &str, prefix: &str) -> Vec<String> {
    let url = pointer.replace(prefix, "");
    let url: Vec<&str> = url.split(':').collect();
    let mut coords: Vec<&str> = url[0].split('.').collect();
    coords[0] = &pointer;
    coords.pop();
    coords.reverse();
    coords.iter().map(|s| s.to_string()).collect()
}

pub type MergedUpdates = Vec<Vec<usize>>;

pub fn merge_visited_nodes(
    visited_nodes: Vec<usize>,
    merged: &mut MergedUpdates,
) -> &mut MergedUpdates {
    for (idx, node) in visited_nodes.iter().enumerate() {
        if merged.len() <= idx {
            merged.push(Vec::with_capacity(1));
        }
        let merges = &merged[idx];
        if merges.contains(node) {
            continue;
        }
        merged[idx].push(*node);
    }

    merged
}
