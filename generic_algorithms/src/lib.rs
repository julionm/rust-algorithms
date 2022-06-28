pub fn binary_search(vector: &[u32], number: u32) -> i32 {

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

pub fn selection_sort(mut vec: Vec<usize>) -> Vec<usize> {
    let len = vec.len();
    let mut ranked = vec![];

    for num in 0..len {

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

pub fn quicksort(vec: Vec<usize>) -> Vec<usize> {
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
        let  mut a = vec![1, 4, 5, 6, 7];
        
        assert_eq!(vec_reducer(a), 23);
    }

    #[test]
    fn test_highest_value() {
        let mut s = vec![6, 7, 8, 10, 4, 12, 14, 37, 2];

        assert_eq!(highest(s), 37);
    }

    #[test]
    fn test_quicksort() {
        let vec = vec![17, 5, 14, 8, 9, 6, 32, 11, 21, 3];

        assert_eq!(quicksort(vec), [3, 5, 6, 8, 9, 11, 14, 17, 21, 32]);
    }

}