pub fn compare<T: PartialEq + Ord + Clone>(data: Vec<T>, db_data: Vec<T>) -> Vec<T> {
    let mut data_sorted = data;
    let mut db_data_sorted = db_data;
    data_sorted.sort();
    db_data_sorted.sort();

    let diff: Vec<T> = data_sorted
        .into_iter()
        .filter(|item| !db_data_sorted.contains(item))
        .collect();

    diff
}