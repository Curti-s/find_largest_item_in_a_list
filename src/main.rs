fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_element<T: PartialOrd> (list: &[T]) -> &T {
    let mut index = 0;
    let mut largest = &list[index];
    let it = list.len() - 1;

    while index <= it {
        if index + 1 < it {
            largest = if list[index] > list[index + 1] {
                &list[index]
            } else {
                &list[index + 1]
            };
        }
        index += 1;
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 20, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y','k', 'j', 'l'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let result = largest_element(&number_list);
    println!("The largest element is {}", result);
    
    let char_list = vec!['y','k', 'j', 'l'];
    let result = largest_element(&char_list);
    println!("The largest char element is {}", result);
}
