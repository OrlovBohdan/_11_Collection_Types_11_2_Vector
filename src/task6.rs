#[test]

/*
// FIX the errors
fn main() {
    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), __);
    assert_eq!(vec.capacity(), 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), __);
    assert_eq!(vec.capacity(), __);

    // ...but this may make the vector reallocate
    vec.push(11);
    assert_eq!(vec.len(), 11);
    assert!(vec.capacity() >= 11);


    // Fill in an appropriate value to make the `for` done without reallocating
    let mut vec = Vec::with_capacity(__);
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), __);
    assert_eq!(vec.capacity(), __);

    println!("Success!");
}
*/

fn main() {
    let mut vec = Vec::with_capacity(10);

    // Вектор не содержит элементов, хотя у него есть место для большего
    assert_eq!(vec.len(), 0); // длина вектора равна 0, так как в нем нет элементов
    assert_eq!(vec.capacity(), 10); // емкость вектора равна 10

    // Эти операции выполняются без перераспределения памяти...
    for i in 0..10 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10); // длина вектора теперь равна 10
    assert_eq!(vec.capacity(), 10); // емкость вектора осталась 10

    // ...но эта операция может вызвать перераспределение вектора
    vec.push(11);
    assert_eq!(vec.len(), 11); // длина вектора теперь равна 11
    assert!(vec.capacity() >= 11); // емкость вектора должна быть как минимум 11


    // Заполните соответствующее значение, чтобы цикл `for` завершился без перераспределения
    let mut vec = Vec::with_capacity(100); // емкость 100, чтобы можно было добавить 100 элементов
    for i in 0..100 {
        vec.push(i);
    }

    assert_eq!(vec.len(), 100); // длина вектора теперь равна 100
    assert_eq!(vec.capacity(), 100); // емкость вектора также равна 100

    println!("Success!");
}


/*
Первый assert_eq!(vec.len(), 0);: В начале вектор пустой, поэтому его длина равна 0.
assert_eq!(vec.capacity(), 10);: Мы заранее задали емкость вектора 10.
После добавления 10 элементов: Длина вектора становится 10, и емкость остается 10.
После добавления 11-го элемента: Длина становится 11, а емкость может увеличиться (поэтому проверяем, что она больше или равна 11).
Создание нового вектора с Vec::with_capacity(100);: Это позволяет без перераспределения добавить 100 элементов.
В конечном итоге assert_eq!(vec.len(), 100); и assert_eq!(vec.capacity(), 100);: Длина и емкость будут равны 100.
*/