fn main() {
    let mut src: Vec<i32> = vec![5, 6, 9, 2, 3];
    let src = normal_bubble_sort(&mut src);
    println!("{:?}", src);

    let mut partial_ord_src = vec![String::from("Bob"), String::from("Alice"), String::from("Rust"), String::from("Go")];
    let partial_ord_src = partial_ord_sort(&mut partial_ord_src);
    println!("{:?}", partial_ord_src);
}


fn normal_bubble_sort(src: &mut Vec<i32>) -> &mut Vec<i32> {
    for item in (0..src.len()).rev() {
        for i in 0..item {
            if src[i] > src[i + 1] {
                src.swap(i, i + 1);
            }
        }
    }
    src
}

fn partial_ord_sort(src: &mut Vec<String>) -> &Vec<String> {
    src.sort_by(|a, b| a.len().cmp(&b.len()));
    src
}





