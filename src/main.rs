use core::slice;

fn max_heap(mut arr: Vec<i32>, index: usize) -> Vec<i32> {
    let l = 2 * index;
    let r = 2 * index + 1;
    let mut largest: usize = 0;

    if l <= (arr.len() - 1) && arr[l] > arr[index] {
        largest = l;
    } else {
        largest = index;
    }

    if r <= (arr.len() - 1) && arr[r] > arr[largest] {
        largest = r;
    }

    if largest != index {
        arr.swap(index, largest);
        arr = max_heap(arr, largest);
    }
    arr
}

fn build_heap(mut arr: Vec<i32>) -> Vec<i32> {
    let heap_size = arr.len();
    for i in 0..((heap_size / 2) + 1) {
        let idx = (heap_size / 2) - i;
        arr = max_heap(arr, idx)
    }
    arr
}

fn heap_sort(mut arr: Vec<i32>) -> Vec<i32> {
    let ptr = &mut arr[0] as *mut i32;
    let arr_len = arr.len();
    arr = build_heap(arr);
    println!("init heap {:?}", arr);
    let mut len = arr_len;
    
    for i in 0..(arr_len - 1) {
        println!("{:?}", arr_len - 1 - i);
        arr.swap(0, arr_len - 1 - i);
        println!("get max heap {:?}", arr);

        len -= 1;
        unsafe {
            arr = slice::from_raw_parts(ptr, len).to_vec();
        }
        println!("new resized heap {:?}", arr);
        arr = max_heap(arr, 0);
        println!("new init heap {:?}", arr);
    }
    unsafe {
        arr = slice::from_raw_parts(ptr, arr_len).to_vec();
    }
    arr
}

fn main() {
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    data = heap_sort(data);
    println!("{:?}", data);
}
