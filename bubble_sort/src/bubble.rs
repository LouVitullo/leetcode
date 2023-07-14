pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j+1)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [4, 2, 9, 5, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 4, 5, 9]);

        let mut empty_arr: [i32; 0] = [];
        bubble_sort(&mut empty_arr);
        assert_eq!(empty_arr, []);

        let mut sorted_arr = [1, 2, 3, 4, 5];
        bubble_sort(&mut sorted_arr);
        assert_eq!(sorted_arr, [1, 2, 3, 4, 5]);

        let mut reverse_arr = [5, 4, 3, 2, 1];
        bubble_sort(&mut reverse_arr);
        assert_eq!(reverse_arr, [1, 2, 3, 4, 5]);
    }
}
