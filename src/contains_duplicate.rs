use std::collections::HashSet; 

pub fn contains_duplicate(x: &[i32]) -> bool {
    if x.len() < 2 {
        return false;
    }

    let mut seen_items = HashSet::new();

    for item in x {
        if seen_items.contains(item) {
            return true;
        }

        seen_items.insert(item);
    }

    return false;
}

#[cfg(test)]
mod contains_duplicate {
    #[test]
    fn empty_list() {
        let list = &[];
        assert!(!super::contains_duplicate(list));
    }

    #[test]
    fn single_element_list() {
        let list = &[ 1 ];
        assert!(!super::contains_duplicate(list));
    }

    #[test]
    fn unique_double_element_list() {
        let list = &[ 1, 2 ];
        assert!(!super::contains_duplicate(list));
    }

    #[test]
    fn duplicate_double_element_list() {
        let list = &[ 1, 1 ];
        assert!(super::contains_duplicate(list));
    }
}
