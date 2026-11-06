fn main() {
    let number = 8;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let mut count = 0;
    'counting_up_loop: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");

            if remaining == 9 {
                break println!("remaining upped = {count}");
            }

            if count == 2 {
                break 'counting_up_loop;
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let a: [i32; 5] = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value of element is: {}", element);
    }

    for number in (1..5) {
        println!("the value of number is: {}", number);
    }
}
