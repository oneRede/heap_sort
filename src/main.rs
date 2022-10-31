fn max_heap(mut arr: Vec<i32>, index: usize) -> Vec<i32> {
    let l = 2 * index + 1;
    let r = 2 * index + 2;
    let mut largest: usize = index;

    if l <= (arr.len() - 1) && arr[l] > arr[index] {
        largest = l;
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

fn max_heap_v2(mut arr: Vec<i32>, index: usize, size: usize) -> Vec<i32> {
    let l = 2 * index + 1;
    let r = 2 * index + 2;
    let mut largest: usize = index;

    if l <= (size - 1) && arr[l] > arr[index] {
        largest = l;
    }

    if r <= (size - 1) && arr[r] > arr[largest] {
        largest = r;
    }

    if largest != index {
        arr.swap(index, largest);
        arr = max_heap_v2(arr, largest, size);
    }
    arr
}

fn build_heap(mut arr: Vec<i32>) -> Vec<i32> {
    let heap_size = arr.len();
    for i in 0..(heap_size / 2) {
        let idx = (heap_size / 2) - i - 1;

        arr = max_heap(arr, idx)
    }
    arr
}

fn build_heap_v2(mut arr: Vec<i32>) -> Vec<i32> {
    let heap_size = arr.len();
    for i in 0..(heap_size / 2) {
        let idx = (heap_size / 2) - i - 1;

        arr = max_heap_v2(arr, idx, heap_size)
    }
    arr
}

fn heap_sort(mut arr: Vec<i32>) -> Vec<i32> {
    arr = build_heap_v2(arr);
    let len = arr.len();
    let mut heap_size = len;
    arr.swap(0, len-1);
    for _i in 0..(len-1){
        if heap_size == 1{
            break
        }
        heap_size -= 1;
        arr = max_heap_v2(arr, 0,heap_size);
        arr.swap(0, heap_size-1);
    }
    arr
}

fn heap_sort_2(mut data: Vec<i32>) {
    let len = data.len();
    data = build_heap(data);

    for _i in 0..(len) {
        if data.len() == 1 {
            continue;
        }
        let _n = data.remove(0);
        data = max_heap(data, 0);
    }
}

fn main() {
    let mut data = vec![400, 1, 3, 2, 5, 6, 7, 8, 9];

    data = heap_sort(data);
    println!("{:?}", data);
    heap_sort_2(data)
}
