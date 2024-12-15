use serde::Serialize;

pub fn compare_objects<T: Clone + Serialize>(data: T, db_data: T) -> Option<T> {
    let data_json = serde_json::to_string(&data).unwrap();
    let db_data_json = serde_json::to_string(&db_data).unwrap();

    if data_json != db_data_json {
        Some(data)
    } else {
        None
    }
}