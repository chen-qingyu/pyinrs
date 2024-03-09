pub fn check_bounds(pos: i32, begin: i32, end: i32) {
    if pos < begin || pos >= end {
        panic!("Error: Index out of range.");
    }
}

pub fn check_empty(size: i32) {
    if size == 0 {
        panic!("Error: The container is empty.");
    }
}

pub fn check_full(size: i32, capacity: i32) {
    if size >= capacity {
        panic!("Error: The container has reached the maximum size.");
    }
}

pub fn calc_index(mut index: i32, size: usize) -> usize {
    if index < 0 {
        index += size as i32;
    }
    index as usize
}
