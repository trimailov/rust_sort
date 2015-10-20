fn main() {
    let mut arr = [8, 1, 2, 0, 9, 4, 7, 6, 3, 5];
    bubble_sort(&mut arr);

    for n in &arr {
        println!("{}", n);
    }
}

fn bubble_sort(arr: &mut [i32]) {
    loop {
        let mut swapped = false;
        let length = arr.len() - 1;

        for i in 0..length {
            if arr[i+1] < arr[i] {
                arr.swap(i, i+1);
                swapped = true;
            }
        }
        if swapped == false {
            break;
        }
    }
}
