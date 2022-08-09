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
}
