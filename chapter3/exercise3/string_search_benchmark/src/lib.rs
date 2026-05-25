#[derive(Debug)]
#[allow(dead_code)]
enum ResultType {
    Boolean(bool),
    OptUSize(Option<usize>),
}

fn print(haystack: &str, needle: &str, result: ResultType) {
    println!(
        "Needle '{:?}' in a Haystack '{:?}' is found {:?}",
        &needle, &haystack, result
    );
}

pub fn search_contains(haystack: &str, needle: &str, is_logged: bool) -> bool {
    let is_found = haystack.contains(needle);
    if is_logged {
        print(haystack, needle, ResultType::Boolean(is_found));
    }
    is_found
}

pub fn search_find(haystack: &str, needle: &str, is_logged: bool) -> Option<usize> {
    let is_found = haystack.find(needle);
    if is_logged {
        print(haystack, needle, ResultType::OptUSize(is_found));
    }
    is_found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haystack_contain_needle() {
        let haystack = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let needle: &str = "PQR";

        assert!(search_contains(&haystack, needle, true));
    }

    #[test]
    fn test_needle_found_in_haystack() {
        let haystack = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let needle: &str = "PQR";

        assert_eq!(search_find(&haystack, needle, true), Some(15));
    }
}
