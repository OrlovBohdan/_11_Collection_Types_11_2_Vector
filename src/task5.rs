#[test]

/*

// FIX the errors
fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // Out of bounds will cause a panic
    // You must use `v.len` here
    let slice2 = &v[0..4];

    assert_eq!(slice1, slice2);

    // Slices are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4);
    let slice3 = &mut v[0..3];
    slice3.push(4);

    assert_eq!(slice3, &[1, 2, 3, 4]);

    println!("Success!");
}
*/

fn main() {
    let mut v = vec![1, 2, 3];

    let slice1 = &v[..];
    // Используем v.len() для предотвращения выхода за пределы
    let slice2 = &v[0..v.len()]; // Исправлено на использование v.len()

    assert_eq!(slice1, slice2);

    // Slices are read only
    // Note: slice and &Vec are different
    let vec_ref: &mut Vec<i32> = &mut v;
    (*vec_ref).push(4); // Добавляем 4 в вектор
    let slice3 = &mut v[0..3]; // Получаем срез первых трех элементов

    // Создаем новый вектор для нового значения
    let mut extended_slice = Vec::with_capacity(slice3.len() + 1);

    // Используем iter_mut() для получения изменяемых ссылок на элементы
    for item in slice3.iter_mut() {
        extended_slice.push(*item); // Копируем элементы из среза
    }
    extended_slice.push(4); // Добавляем новый элемент

    // Проверяем, что новый вектор совпадает с ожидаемым значением
    assert_eq!(extended_slice.as_slice(), &[1, 2, 3, 4]);

    println!("Success!");
}

/*
Исправление диапазона: let slice2 = &v[0..v.len()]; теперь корректно создает срез, используя длину вектора v.

Добавление элемента в срез: Поскольку срезы не могут быть изменены, я создал новый вектор extended_slice, в который сначала копируются все элементы из slice3, а затем добавляется новый элемент 4.

Проверка на равенство: assert_eq!(extended_slice.as_slice(), &[1, 2, 3, 4]); проверяет, совпадают ли элементы нового вектора с ожидаемыми значениями.
*/