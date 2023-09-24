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

fn is_a_sublist_of_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {


    return true;
}

fn is_a_superlist_of_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {


    return true;
}

fn is_a_equal_to_b<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    if a.len() != b.len() { return false }

    return a.iter().eq(b.iter());
}
