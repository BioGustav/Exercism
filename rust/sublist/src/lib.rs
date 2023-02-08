#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;

    return match (first_list.len(), second_list.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (l0, l1) if l0 > l1 => if first_list.windows(l1).any(|w| w == second_list) {Superlist} else {Unequal},
        (l0, l1) if l0 < l1 => if second_list.windows(l0).any(|w| w == first_list) {Sublist} else {Unequal},
        (_, _) => if first_list == second_list {Equal} else {Unequal},
    };
}

pub fn _sublist_old<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let (len0, len1) = (first_list.len(), second_list.len());
    if len0 == len1 {
        return if first_list == second_list {
            Comparison::Equal
        } else {
            Comparison::Unequal
        };
    }

    let (result, list0, list1) = if first_list.len() < second_list.len() {
        (Comparison::Sublist, first_list, second_list)
    } else {
        (Comparison::Superlist, second_list, first_list)
    };

    let (len0, len1) = (list0.len(), list1.len());
    let mut offset = 0;

    while len0 + offset != len1 + 1 {
        if list0[0..len0] == list1[offset..offset + len0] {
            return result;
        }
        offset += 1;
    }
    Comparison::Unequal
}
