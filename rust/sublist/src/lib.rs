#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    use Comparison::*;

    match (_first_list.len(), _second_list.len()) {
        (0,0) => Equal,
        (0,_) => Sublist,
        (_,0) => Superlist,
        (a,b) if a > b => if _first_list
            .windows(b)
            .any(|el| el == _second_list) { Superlist } else { Unequal },
        (a,b) if a < b => if _second_list
            .windows(a)
            .any(|el| el == _first_list) { Sublist } else { Unequal },
        (_,_) => if _first_list == _second_list {
            Equal
        } else {
            Unequal
        }
    }
}