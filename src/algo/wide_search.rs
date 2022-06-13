pub fn search(items: &[i32], _value : i32)-> Option<i32>{
    let size = items.len();

    for i in 0..size {
        if items[i] == _value{
            return Some(0)
        }
    }

    None
}