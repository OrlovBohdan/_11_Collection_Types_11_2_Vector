#[test]

/*
#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    // FILL in the blank
    let v : Vec<IpAddr>= __;

    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}
*/

fn main() {
    // Заполняем вектор `v` с элементами типа `IpAddr`
    let v: Vec<IpAddr> = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];

    // Сравниваем два перечисления; необходимо вывести трейт PartialEq
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}


#[derive(Debug, PartialEq)] // Добавляем PartialEq для сравнения
enum IpAddr {
    V4(String),
    V6(String),
}


/*
PartialEq: Добавление PartialEq в #[derive(Debug, PartialEq)] позволяет использовать assert_eq!,
чтобы сравнивать значения перечислений. Это необходимо, так как Rust не может автоматически
сравнивать разные типы, если не реализован соответствующий трейт.

Инициализация v: Вектор v инициализируется с двумя элементами типа IpAddr: один для IPv4 и один для IPv6.
*/