pub fn find<T : Ord , U : AsRef<[T]>>(array: U, key: T) -> Option<usize> {
    let array = array.as_ref();
    let mut left: i32 = 0;
    let mut right = (array.len() as i32 - 1) as i32;
    while right >= left {
        let mid = left + (right - left)/2;
        if array[mid as usize] == key {return Some(mid as usize);}
        if array[mid as usize] > key {right = mid - 1;}
        else {left = mid + 1;}
    }
    None
}
