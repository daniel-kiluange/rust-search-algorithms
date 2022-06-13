pub fn search(items: &[i32], _value: i32) -> Option<i32>{

    if items.is_empty() {
        return None;
    }

    let middle = (items.len() - 1) / 2;
    
    if items[middle] == _value {
        return Some(0);
    }
    if items[middle] > _value {
        return search(&items[..middle], _value);
    }

    if items[middle] < _value {
        return search(&items[middle+1..], _value);
    }

    None
}