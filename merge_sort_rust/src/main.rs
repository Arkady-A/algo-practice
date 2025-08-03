fn main() {
    let input_data = &[0, 5, 4, 23, 1, 3];

    let result = merge_sort(input_data);
    let result_str: String = result
        .into_iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", result_str);
}

fn merge_sort<T: Ord + Clone>(array: &[T]) -> Vec<T> {
    let array_length = array.len();

    if array_length > 1 {
        let middle = array_length / 2;

        let left: Vec<T> = merge_sort(&array[..middle]);
        let right: Vec<T> = merge_sort(&array[middle..]);

        merge(&left, &right)
    } else {
        vec![array[0].clone()]
    }
}

fn merge<T: Ord + Clone>(left_array: &[T], right_array: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left_array.len() + right_array.len());
    let mut i = 0;
    let mut j = 0;

    while i < left_array.len() && j < right_array.len() {
        if left_array[i] >= right_array[j] {
            result.push(left_array[i].clone());
            i += 1
        } else {
            result.push(right_array[j].clone());
            j += 1
        }
    }

    result.extend_from_slice(&left_array[i..]);
    result.extend_from_slice(&right_array[j..]);
    result
}
