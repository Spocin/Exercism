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

    return a == b;
}

fn is_a_sublist_of_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() == 0 { return true }

    let a_deduped = dedup_array(&a);
    let b_deduped = dedup_array(&b);

    let el_idx_in_b = b_deduped.iter().position(|b_el| b_el == &a_deduped[0]);

    if el_idx_in_b.is_some() {
        let el_idx_in_b_val = el_idx_in_b.unwrap();

        let trailing_len_of_b = b_deduped.len() - el_idx_in_b_val;
        if a_deduped.len() <= trailing_len_of_b {
            let b_idx_to_slice_to = el_idx_in_b_val + a_deduped.len();

            return is_arr_a_equal_to_arr_b(&a_deduped, &b_deduped[el_idx_in_b_val..b_idx_to_slice_to]);
        }
    }

    return false;
}

fn is_a_superlist_of_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if b.len() == 0 { return true }

    is_a_sublist_of_b(&b, &a)
}

fn dedup_array<T: PartialEq>(arr: &[T]) -> Vec<&T> {
    let mut vec_from_arr = arr.iter().collect::<Vec<_>>();
    vec_from_arr.dedup();

    return vec_from_arr;
}