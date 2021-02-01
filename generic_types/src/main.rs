fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point {
        x: 5,
        y: 10,
    };

    let float = Point {
        x: 1.0,
        y: 4.0,
    };

    // let integer_float = Point {
    //     x: 1,
    //     y: 2.0,
    // };

    let p = Point {
        x: 5,
        y: 10,
    };
    println!("p.x = {}", p.x());
}

//
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

struct Points<T, U> {
    x: T,
    y: U,
}

impl<T, U> Points<T, U> {
    fn mixup<V, W>(self, other: Points<V, W>) -> Points<T, W> {
        Points {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}