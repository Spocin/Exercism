#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if is_a_equal_to_b(_first_list,_second_list) { return Comparison::Equal; }
    if is_a_sublist_of_b(_first_list,_second_list) { return Comparison::Sublist; }
    if is_a_superlist_of_b(_first_list,_second_list) { return Comparison::Superlist; }

    Comparison::Unequal
}

fn is_a_equal_to_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() { return false }

    return a.iter().eq(b.iter());
}

fn is_a_sublist_of_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() == 0 { return true }

    let el_idx_in_b = b.iter().position(|b_el| b_el == &a[0]);

    if el_idx_in_b.is_some() {
        let el_ind_in_b_val = el_idx_in_b.unwrap();

        //Check for length overflow
        let trailing_len_of_b = b.len() - el_ind_in_b_val;
        if a.len() <= trailing_len_of_b {
            return is_a_equal_to_b(&a, &b[el_ind_in_b_val..a.len()])
        }
    }

    return false;
}

fn is_a_superlist_of_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if b.len() == 0 { return true }

    return false;
}