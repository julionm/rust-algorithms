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
}