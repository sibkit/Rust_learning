mod user;

fn test_fmt()
{
    let s:String = format!["Hello {}!","мир"];
    eprint!("[ERROR] ");
    println!("{}",s);


    println!("{} дней", 31);
    println!("{0}, это {1}. {1}, это {0}", "Алиса", "Боб");

    // Так же можно именовать аргументы.
    println!("{subject} {verb} {object}",
             object="ленивую собаку",
             subject="быстрая коричневая лиса",
             verb="прыгает через");

    println!("{} из {:b} людей знают, что такое двоичный код, а остальные нет.", 1, 2);

    // Можно выравнивать текст, сдвигая его на указанную ширину.
    // Данный макрос отобразит в консоли
    // "     1". 5 пробелов и "1".
    println!("{number:>width$}", number=1, width=6);

    // Можно добавить к цифрам пару нулей. Данный макрос выведет "000001".
    println!("{number:0>width$}", number=1, width=6);

    // Компилятор обязательно проверит, что в макрос передано правильное количество
    // аргументов.
    println!("Меня зовут {1}, {0} {1}", "Джеймс","Бонд");
    // ИСПРАВЬТЕ ^ Добавьте недостающий аргумент: "Джеймс"

    // Создаём структуру, которая хранит в себе `i32`. Назовём её `Structure`.


    // Однако, пользовательские типы данных, например, как эта структура
    // требуют более сложной обработки для вывода. Данный код не будет работать.
    //println!("Эта структура `{}` не хочет выводится на экран...", Structure(3));
    // ИСПРАВЬТЕ ^ Закомментируйте эту строку.

    let pi = std::f64::consts::PI;// 3.141592;
    println!("Pi is {pi}");
    println!("Pi is roughly {pi:.2}");
    println!("{:#?}",(4,pi,5));

    let m = 1_000_000f64;
    println!("M is {m:.*}",3);
}

fn main()
{
    //test_fmt();
    let user = user::User
    {
        login: String::from("first user"),
        name: "Dasha".to_string(),
        email: "".to_string(),
        sign_in_count: 0
    };
    println!("{}",user);
    println!("Hello {}",user.name);

    let mut b = ("Vasya", 5,(01,02,03u8,"some text",std::f64::consts::PI*2f64), 6.98,"tyrkmenia is country",6,7,8,9,0,1,2);
    let c = (05,06,07u8,"second text", std::f64::consts::TAU);
    println!("b is {:?}",b);



    if b.1<25
    {
        b.2 = c;
        println!("b is {:?}",b);
    }
    else { println!("b is {}", b.1) }

}
