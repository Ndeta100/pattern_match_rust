fn main() {
    let fovorite_color: Option<&str> = None;
    let age: Result<u8, _> = "34".parse();
    let is_tuesday = true;
    if let Some(color) = fovorite_color {
        println!("Using your favorite color {}, is as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("using blue as the background color");
    }
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top)
    }
    let v = vec!['a', 'f', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
    let x = 5;
    let (x, y, z) = (1, 23, 4);
    let point = (3, 5);
    print_ccoordinates(&point);
    //PATTERN SYNTHAX
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    let y = 1;
    let v = Some(67);
    match v {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y={:?}", y),
        _ => println!("Default case, v={:?}", v),
    }
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    let msg = Message::ChangeColor(Color::HSV(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("the Quit variant has no data to destructure")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y directionm {}",
                x, y
            )
        }
        Message::Write(text) => {
            println!("Text message {}", text)
        }
        Message::ChangeColor(Color::HSV(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {} and value {}",
                h, s, v
            )
        }
        Message::ChangeColor(Color::RGB(r, g, b)) => {
            println!("Change the color to red {}, green {} and blue {}", r, g, b)
        }
    }
}
fn multiple_pattern() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}
fn matches_ranges() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}
fn print_ccoordinates(&(x, y): &(i32, i32)) {
    println!("Current location : ({}, {}", x, y);
}
fn destructuure_structs_and_tuples() {}
struct Point {
    x: i32,
    y: i32,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
enum Color {
    RGB(i32, i32, i32),
    HSV(i32, i32, i32),
}
fn destructuring_structs() {
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
