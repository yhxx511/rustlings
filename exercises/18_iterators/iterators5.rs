// iterators5.rs
//
// Let's define a simple model to track Rustlings exercise progress. Progress
// will be modelled using a hash map. The name of the exercise is the key and
// the progress is the value. Two counting functions were created to count the
// number of exercises with a given progress. Recreate this counting
// functionality using iterators. Try not to use imperative loops (for, while).
// Only the two iterator methods (count_iterator and count_collection_iterator)
// need to be modified.
//
// Execute `rustlings hint iterators5` or use the `hint` watch subcommand for a
// hint.



use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
// TODO 直接通过derive来从Debug继承，Debug虽然只是一个trait，但所有的结构体都能直接进行toString了
enum Progress {
    None,
    Some,
    Complete,
}

// impl Display for Progress {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Progress::None => write!(f, "None"),
//             Progress::Some => write!(f, "Some"),
//             Progress::Complete => write!(f, "Complete"),
//         }
//     }
// }
//
// impl Debug for Progress {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Progress::None => write!(f, "None"),
//             Progress::Some => write!(f, "Some"),
//             Progress::Complete => write!(f, "Complete"),
//         }
//     }
// }

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

// filter的实现方式
fn count_iterator_by_filter(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map is a hashmap with String keys and Progress values.
    // map = { "variables1": Complete, "from_str": None, ... }
    map.into_iter()
        .filter(|x| *x.1 == value)
        .fold(0, |acc, x| acc + 1)
}

// match表达式的实现
// TODO match的实现方式走不通：value是个变量，不能作为match的pattern进行匹配
//      因为match的arm必须是一个静态的东西，不能是动态求值的。
// fn count_iterator_by_match(map: &HashMap<String, Progress>, value: Progress) -> usize {
//     map.into_iter().fold(0, |acc, x:(&String, &Progress)|
//         {match x.1 {
//             Progress@value => { println!("cc  {}, {}, expected {}", x.0, x.1, value); acc + 1 },
//             _ => { acc }
//         }})
// }

// 使用if实现的版本
fn count_iterator_by_if(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.into_iter().fold(0, |acc, x| {
        if *(x.1) == value { acc + 1 } else {acc}
    })
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection is a slice of hashmaps.
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    // 解释：
    // map函数，为每个子map求count，得到一个usize
    // fold函数，为每个子map得到的结果进行求和
    let v1 = collection.into_iter()
        .map(|some_map| count_iterator_by_filter(some_map, value))
        .fold(0, |acc, x| acc + x);
    // let v2 = collection.into_iter()
    //     .map(|some_map| count_iterator_by_match(some_map, value))
    //     .fold(0, |acc, x| acc + x);
    let v3 = collection.into_iter()
        .map(|some_map| count_iterator_by_if(some_map, value))
        .fold(0, |acc, x| acc + x);
    assert_eq!(v1, v3);
    // assert_eq!(v2, v3);
    v3
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator_by_filter(&map, Progress::Complete));
        assert_eq!(3, count_iterator_by_if(&map, Progress::Complete));
        // assert_eq!(3, count_iterator_by_match(&map, Progress::Complete));
    }

    #[test]
    fn count_some() {
        let map = get_map();
        assert_eq!(1, count_iterator_by_filter(&map, Progress::Some));
        assert_eq!(1, count_iterator_by_if(&map, Progress::Some));
        // assert_eq!(1, count_iterator_by_match(&map, Progress::Some));
    }

    #[test]
    fn count_none() {
        let map = get_map();
        assert_eq!(2, count_iterator_by_filter(&map, Progress::None));
        assert_eq!(2, count_iterator_by_if(&map, Progress::None));
        // assert_eq!(2, count_iterator_by_match(&map, Progress::None));
    }

    #[test]
    fn count_complete_equals_for() {
        let map = get_map();
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        for progress_state in progress_states {
            assert_eq!(count_for(&map, progress_state), count_iterator_by_filter(&map, progress_state));
            assert_eq!(count_for(&map, progress_state), count_iterator_by_if(&map, progress_state));
            // assert_eq!(count_for(&map, progress_state), count_iterator_by_match(&map, progress_state));
        }
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_some() {
        let collection = get_vec_map();
        assert_eq!(1, count_collection_iterator(&collection, Progress::Some));
    }

    #[test]
    fn count_collection_none() {
        let collection = get_vec_map();
        assert_eq!(4, count_collection_iterator(&collection, Progress::None));
    }

    #[test]
    fn count_collection_equals_for() {
        let progress_states = vec![Progress::Complete, Progress::Some, Progress::None];
        let collection = get_vec_map();

        for progress_state in progress_states {
            assert_eq!(
                count_collection_for(&collection, progress_state),
                count_collection_iterator(&collection, progress_state)
            );
        }
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
