#[test]

/*

// FILL in the blanks
fn main() {
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = __(arr);
    let v2: Vec<i32> = arr.__();

    assert_eq!(v1, v2);


    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.__();

    let s = "hello".to_string();
    let v2 = s.into_bytes();
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::__(s);
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect();
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");
 }
*/


fn main() {
    // Array -> Vec
    // impl From<[T; N]> for Vec
    let arr = [1, 2, 3];
    let v1 = Vec::from(arr); // Используем `Vec::from` для преобразования массива в вектор
    let v2: Vec<i32> = arr.to_vec(); // Используем `to_vec` для преобразования массива в вектор

    assert_eq!(v1, v2);

    // String -> Vec
    // impl From<String> for Vec
    let s = "hello".to_string();
    let v1: Vec<u8> = s.into_bytes(); // Используем `into_bytes` для преобразования строки в вектор байтов

    let s = "hello".to_string();
    let v2 = s.into_bytes(); // `into_bytes` возвращает `Vec<u8>` из `String`
    assert_eq!(v1, v2);

    // impl<'_> From<&'_ str> for Vec
    let s = "hello";
    let v3 = Vec::from(s); // Используем `Vec::from` для преобразования строки в вектор байтов
    assert_eq!(v2, v3);

    // Iterators can be collected into vectors
    let v4: Vec<i32> = [0; 10].into_iter().collect(); // Используем `collect` для сбора значений из итератора
    assert_eq!(v4, vec![0; 10]);

    println!("Success!");
}

/*
Vec::from(arr): Этот метод используется для преобразования массива в вектор.
arr.to_vec(): Этот метод также преобразует массив в вектор и является альтернативным способом.
s.into_bytes(): Метод into_bytes преобразует строку в вектор байтов.
Vec::from(s): Этот метод используется для преобразования строкового среза в вектор байтов.
*/