#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
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
