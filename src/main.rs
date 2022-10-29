use std::ptr;

fn heap_sort(data: *mut i32, len: usize) -> *mut i32 {
    for i in 0..(len - 1) {
        
    }
    data
}

fn main() {
    println!("Hello, world!");
    let mut data = vec![1, 2, 3, 4, 6, 7, 8, 9, 5];
    let ptr = &mut data[0] as *mut i32;
    heap_sort(ptr, 9);
}
