#[derive(Debug)]
enum IpAddKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddKind,
    address: String,
}

// or we can use enum with variable
// variable can be anything even a struct
enum IpAddKindWithValue {
    V4(String),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}



fn route(ip_type: IpAddKind) {}


fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    let home = IpAddr {
        kind: IpAddKind::V6,
        address: String::from("127.0.0.1"),
    };

    let home = IpAddKindWithValue::V6(String::from("127.0.0.1"));

    // introducing options

    let some_num = Some(5);
    let some_string = Some("a string");

    let mapped_value = some_num.map(|v| v * 2 );

    mapped_value.or_else(|| Some(5));

    None.unwrap_or("bike");

    println!("{:?}", mapped_value);
}

