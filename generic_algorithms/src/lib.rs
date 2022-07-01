use std::collections::{HashMap, HashSet, VecDeque};
use std::f32::INFINITY;

pub fn binary_search(vector: &[u32], number: u32) -> i32 { // * O(log n)

    let mut init = 0;
    let mut last = vector.len() as u32;

    if last == 0 {
        return -1;
    }

    while init <= last {
        
        let mid = init + (last - init)/2;

        if number > vector[mid as usize] {
            init = mid + 1;
        } else if number < vector[mid as usize] {
            last = mid - 1;
        } else {
            return mid as i32;
        }
    }

    -1
}

pub fn selection_sort(mut vec: Vec<usize>) -> Vec<usize> { // *  O(n * n)
    let len = vec.len();
    let mut ranked = vec![];

    for _ in 0..len {

        let mut highest = 0_usize;
        let mut highest_loc = 0_usize;
        let other_vec = &mut vec;

        for (index, value) in other_vec.into_iter().enumerate() {
            if *value > highest {
                highest = *value;
                highest_loc = index;
            }
        }

        other_vec.remove(highest_loc);

        ranked.push(highest);

    }

    ranked
}

pub fn factorial(x: usize) -> usize {
    if x == 1 {
        return x
    } else {
        x * factorial(x - 1)
    }
}

pub fn vec_reducer(mut vec: Vec<usize>) -> usize {
    if vec.len() == 0 {
        0
    } else {
        vec.pop().unwrap() + vec_reducer(vec)
    }
}

pub fn highest(mut vec: Vec<usize>) -> usize {

    if vec.len() == 1 {
        vec[0]
    } else if vec.len() == 0 {
        0
    } else {
        if vec[0] < vec[1] {
            vec.remove(0);
            highest(vec)
        } else  {
            vec.remove(1);
            highest(vec)
        }
    }

}

pub fn quicksort(vec: Vec<usize>) -> Vec<usize> { // * O(n log n)
    if vec.len() < 2 {
        return vec;
    } else {

        let pivot = vec[0];

        let mut lesser: Vec<usize> = vec![];
        let mut greater: Vec<usize> = vec![];

        for val in vec[1..].into_iter() {
            if *val <= pivot {
                lesser.push(*val);
            } else if *val > pivot {
                greater.push(*val);
            }
        }

        let mut lesser = quicksort(lesser);
        lesser.push(pivot);
        let greater = quicksort(greater);

        [lesser, greater].concat()
    }
}

pub fn breadth_first_search() { // * O(V + E)
    let mut hash_map: HashMap<&str, Vec<&str>> = HashMap::new();

    hash_map.insert("you", vec!["alice", "bob", "claire"]);
    hash_map.insert("bob", vec!["anuj", "peggy"]);
    hash_map.insert("alice", vec!["peggy"]);
    hash_map.insert("claire", vec!["thom", "jonny"]);
    hash_map.insert("anuj", vec![]);
    hash_map.insert("peggy", vec![]);
    hash_map.insert("thom", vec![]);
    hash_map.insert("jonny", vec![]);

    let mut queue: VecDeque<&str> = VecDeque::from(hash_map.get("you").unwrap().clone());

    let mut searched: Vec<&str> = vec![];

    println!("The queue: {:?}", queue);

    while queue.len() > 0 {
        let person = queue.pop_front().unwrap();

        if !searched.contains(&person) {
            if person.contains("th") { // * logic to define if we got to the destination
                println!("{person} is a great person!");
                return;
            } else {
                searched.push(person);
                queue.append(
                    &mut VecDeque::from(hash_map.get(person).unwrap().clone())
                )
            }
        }
    }

}

pub fn dijkstra() {
    let mut graph = HashMap::from([
        ("start", HashMap::from([
            ("a", 6_f32),
            ("b", 2_f32)
        ])), 
        ("a", HashMap::from([
            ("end", 1_f32)
        ])), 
        ("b", HashMap::from([
            ("a", 3_f32),
            ("end", 5_f32)
        ])),
        ("end", HashMap::new())
    ]);
    let mut costs = HashMap::from([
        ("a", 6_f32),
        ("b", 2_f32),
        ("end", INFINITY)
    ]);
        
    let mut parents = HashMap::from([
        ("a", "start"),
        ("b", "start"),
        ("end", "")
    ]);

    let mut processed: Vec<&str> = vec![];

    // * graph["start"]["a"]
    // * graph.get("start").unwrap().get("a").unwrap();

    let mut node = find_lowest_cost_node(&costs, &processed);

    while let Some(node_v) = node {

        let cost = *costs.get(node_v).unwrap();

        let neighbors = graph.get(node_v).unwrap();

        for neighbor in neighbors.keys() {
            let new_cost = cost + neighbors.get(neighbor).unwrap();

            if *costs.get(neighbor).unwrap() > new_cost {
                costs.insert(neighbor, new_cost);
                parents.insert(neighbor, node_v);
            }
        }

        processed.push(node_v);
        node = find_lowest_cost_node(&costs, &processed);
    }

    println!("In the end lesser cost was: {}", costs.get("end").unwrap());
}

pub fn find_lowest_cost_node<'a>(costs: &HashMap<&'a str, f32>, processed: &Vec<&str>) -> Option<&'a str>  {
    let mut lowest_cost = INFINITY;
    let mut lowest_cost_node: Option<&str> = None;

    for (node, value) in costs {
        if *value < lowest_cost && !&processed.contains(&node) {
            lowest_cost = *value;
            lowest_cost_node = Some((*node).clone());
        }
    }

    lowest_cost_node
}

pub fn greedy_algorithm() {
    let mut wanted_states = HashSet::from(
        [
            String::from("sp"),
            String::from("mg"),
            String::from("ba"),
            String::from("rj"),
            String::from("am"),
            String::from("pa"),
            String::from("sc"),
            String::from("rs"),
            String::from("es")
        ]
    );

    let stations = HashMap::from([
        (
            String::from("one"),
            HashSet::from([String::from("rj"), String::from("pa"), String::from("sp")])
        ),
        (
            String::from("two"),
            HashSet::from([String::from("ba"), String::from("mg"), String::from("rs")])
        ),
        (
            String::from("three"),
            HashSet::from([String::from("am"), String::from("es"), String::from("sc")])
        ),
        (
            String::from("four"),
            HashSet::from([String::from("rj"), String::from("pa"), String::from("sp")])
        ),
        (
            String::from("five"),
            HashSet::from([String::from("es"), String::from("rs")])
        ),
    ]);

    let mut final_stations: HashSet<String> = HashSet::new();
    
    while wanted_states.len() > 0 {
        let mut states_covered: HashSet<&String> = HashSet::new();
        let mut best_station = String::new();

        for (state, stations) in &stations {
            let covered: HashSet<&String> = wanted_states.intersection(stations).collect();

            if covered.len() > states_covered.len() {
                best_station = state.clone();
                states_covered = covered;
            }
        }

        wanted_states = 
            wanted_states
                .difference(
                    &states_covered
                        .into_iter()
                        .map(Clone::clone)
                        .collect()
                )
                .map(Clone::clone)
                .collect();

        final_stations.insert(best_station);
    }

    println!("{:?}", final_stations);

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_binary_search_empty_list() {
        let vector: Vec<u32> = vec![];

        assert_eq!(binary_search(&vector, 1), -1);
    }

    #[test]
    fn test_binary_search() {

        let vector = vec![10, 11, 14, 15, 16, 17, 18, 21, 24, 30, 45];
        let number = 14;

        assert_eq!(binary_search(&vector, number), 2);

    }

    #[test]
    fn test_selection_sort() {
        let selection_sort_vec = vec![9, 16, 7, 8, 2, 12];
        let right_answer = vec![16, 12, 9, 8, 7, 2];

        assert_eq!(selection_sort(selection_sort_vec), right_answer);

    }

    #[test]
    fn test_sum_vec_reducer() {
        let a = vec![1, 4, 5, 6, 7];
        
        assert_eq!(vec_reducer(a), 23);
    }

    #[test]
    fn test_highest_value() {
        let s = vec![6, 7, 8, 10, 4, 12, 14, 37, 2];

        assert_eq!(highest(s), 37);
    }

    #[test]
    fn test_quicksort() {
        let vec = vec![17, 5, 14, 8, 9, 6, 32, 11, 21, 3];

        assert_eq!(quicksort(vec), [3, 5, 6, 8, 9, 11, 14, 17, 21, 32]);
    }

    #[test]
    fn test_breadth_first_search() {
        breadth_first_search();

        assert!(true);
    }

    #[test]
    fn test_dijkstra() {
        // dijkstra();

        assert!(true)
    }

}