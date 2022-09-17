#![forbid(unsafe_code)]

use crate::types::{Data, Key};

pub fn new_hashmap(len: usize) -> Vec<Vec<(Key, Data)>> {
    let mut new_hash = Vec::new();
    for _i in 0..len {
        new_hash.push(vec![]);
    }
    new_hash
}

pub fn insert(table: &mut Vec<Vec<(Key, Data)>>, key: Key, value: Data) -> &mut Data {
    if table.is_empty() {
        panic!("insert in empty kolhoz-table");
    }

    let hash_value = key.get_hash() as usize;
    let index = hash_value % table.len();

    table[index].push((key, value));
    let cur_index_size = table[index].len();
    &mut table[index][cur_index_size - 1].1
}

pub fn get_one_or_default<'a, 'b>(
    table: &'a Vec<Vec<(Key, Data)>>,
    key: &'b Key,
    default_value: &'a Data,
) -> &'a Data {
    if table.is_empty() {
        return default_value;
    }

    let hash_value = key.get_hash() as usize;
    let index = hash_value % table.len();
    if table[index].is_empty() {
        default_value
    } else {
        for (cur_key, cur_value) in table[index].iter() {
            if cur_key == key {
                return cur_value;
            }
        }
        default_value
    }
}

pub fn multi_get<'a, 'b>(
    table: &'a Vec<Vec<(Key, Data)>>,
    keys: &'b Vec<Key>,
) -> Vec<(&'b Key, Vec<&'a Data>)> {
    let mut ans_vec: Vec<(&'b Key, Vec<&'a Data>)> = vec![];

    for cur_key in keys {
        if table.is_empty() {
            ans_vec.push((cur_key, vec![]));
            continue;
        }

        let mut add_vec: Vec<&'a Data> = vec![];
        let hash_value = cur_key.get_hash() as usize;
        let cur_index = hash_value % table.len();
        for (key, val) in &table[cur_index] {
            if key == cur_key {
                add_vec.push(val);
            }
        }
        ans_vec.push((cur_key, add_vec));
    }

    ans_vec
}

pub fn find_keys_of_data<'a, 'b>(
    table: &'a Vec<Vec<(Key, Data)>>,
    value: &'b Data,
) -> Vec<&'a Key> {
    let mut ans_vec: Vec<&'a Key> = vec![];

    for bucket in table {
        for (cur_key, cur_value) in bucket {
            if cur_value == value {
                ans_vec.push(cur_key);
            }
        }
    }

    ans_vec
}

pub fn resize(table: &mut Vec<Vec<(Key, Data)>>, new_len: usize) {
    if new_len == table.len() {
        return;
    }

    if new_len == 0 {
        *table = vec![];
        return;
    }

    if table.len() < new_len {
        let add = new_len - table.len();
        for _i in 0..add {
            table.push(vec![]);
        }
    }

    for i in 0..table.len() {
        let mut j: usize = 0;
        while j < table[i].len() {
            let hash_value = table[i][j].0.get_hash() as usize;
            let new_index = hash_value % new_len;

            if new_index == i {
                j += 1;
            } else {
                let (key, value) = table[i].remove(j);
                table[new_index].push((key, value));
            }
        }
    }

    if table.len() > new_len {
        let add = table.len() - new_len;
        for _i in 0..add {
            table.pop();
        }
    }
}
