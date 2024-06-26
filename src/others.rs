// fn test_mut() {
//     let mut s = String::from("Hello");
//     let s_ref = &s;
//     let s_mut = &mut s;
//     println!("s_ref contains text: '{s_ref}'."); // Ошибка! Нельзя использовать иммутабельную ссылку, когда существует мутабельная.
// }

fn foo() {
    // Создаём новую строку. Теперь у этой строки есть перемменная-владелец: `s`.
    let s = String::from("Hello, world!");

    // Передаём строку в функцию в качестве аргумента.
    // При этом владение строкой перемещается в функцию.
    print_string(s);

    // Здесь переменной `s` уже не существует (она перемещена в функцию print_string), поэтому
    // деинициализация не требуется.

    // println!("{s}");
}

// Функция принимает аргумент - строку. При передаче, владение строкой переходит аргументу `s_arg`.
fn print_string(s_arg: String) {
    println!("{s_arg}");

    // Переменная `s_arg` выходит из области видимости в конце функции, следовательно,
    // компилятор разместит здесь код деинициализации строки.
}

// fn borrow_from_scope() {
//     let int_ref = { // Ошибка! Мы пытаемся присвоить этой переменной ссылку, указывающую на уничтоженные данные.
//         let a = return_borrowed; // Этот объект будет деинициализирован в конце блока кода.
//         &a // Эта ссылка будет указывать на уничтоженные данные после завершения блока кода.
//     };
// }

// fn return_borrowed() -> &i32 {
//     let a = 42; // Этот объект будет деинициализирован в конце функции.
//     &a // Ошибка! Мы пытаемся вернуть ссылку, указывающую на данные, которые будут уничтожены после выхода из функции.
// }

fn test_mirror() {
    let mut array = [1, 2, 3, 4, 5];
    mirror(&mut array[..]);
    println!("{:?}", array); // [5, 4, 3, 2, 1]

    mirror(&mut array[1..4]);
    println!("{:?}", array); // [5, 2, 3, 4, 1]

    mirror(&mut array[..2]);
    println!("{:?}", array); // [2, 5, 3, 4, 1]
}

/// Зеркально отражает слайс.
fn mirror(slice: &mut [i32]) {
    let len = slice.len();
    if len == 0 {
        return;
    }

    for i in 0..len / 2 {
        slice.swap(i, len - i - 1)
    }
}

fn print_each_item(slice: &[i32]) {
    for item in slice {
        println!("item: {item}")
    }
}

fn main() {
    print_each_item(&[1, 2, 3, 4]);
}
