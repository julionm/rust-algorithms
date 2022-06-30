use generic_algorithms::dijkstra;

use std::collections::HashMap;

fn main() {
    // dijkstra();

    let mut a = HashMap::from([
        ("a", 5),
        ("b", 2)
    ]);
    let mut b = vec!["d", "e", "f"];

    let n = anything(&a, &b);

    a.push("x");

}

fn anything(a: &HashMap<&str, i32>, b: &Vec<&str>) -> Option<&str> {

    let x: Option<&str> = None;

    for (v, _) in a {
        x = Some(*v);
    }

    x
}

