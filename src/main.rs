fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 20, 100, 65];

    let result = largest(&number_list);
    println!("The largest nuber is {}", result);

    let char_list = vec!['y','k', 'j', 'l'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
