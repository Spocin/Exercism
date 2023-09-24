#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if is_arr_a_equal_to_arr_b(_first_list, _second_list) { return Comparison::Equal; }
    if is_a_sublist_of_b(_first_list,_second_list) { return Comparison::Sublist; }
    if is_a_superlist_of_b(_first_list,_second_list) { return Comparison::Superlist; }

    Comparison::Unequal
}

fn is_arr_a_equal_to_arr_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() { return false }

    a == b
}

fn is_a_sublist_of_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() == 0 { return true }

    let a_deduped = dedup_array(&a);
    let b_deduped = dedup_array(&b);

    //Find all idx to check for
    let found_idx_vec = b_deduped.iter()
        .enumerate()
        .filter(|(_, &el)| el == a_deduped[0])
        .map(|(idx, _)| idx)
        .collect::<Vec<_>>();

    if found_idx_vec.len() == 0 { return false; }

    //Check for each
    for idx in found_idx_vec {
        let trailing_len_of_b = b_deduped.len() - idx;
        if a_deduped.len() > trailing_len_of_b { continue }

        let b_idx_to_slice_to = idx + a_deduped.len();
        let b_slice_to_check = &b_deduped[idx..b_idx_to_slice_to];

        if is_arr_a_equal_to_arr_b(&a_deduped, b_slice_to_check) { return true; }
    }

    false
}

fn is_a_superlist_of_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if b.len() == 0 { return true; }

    is_a_sublist_of_b(&b, &a)
}

fn dedup_array<T: PartialEq>(arr: &[T]) -> Vec<&T> {
    let mut vec_from_arr = arr.iter().collect::<Vec<_>>();
    vec_from_arr.dedup();

    vec_from_arr
}