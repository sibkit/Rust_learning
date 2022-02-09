use std::cmp::Ordering;
use std::io::stdin as cin;
use rand::{Rng};

const ATTEMPTS: u8 = 5;

pub fn run_game()
{
    println!("Угадайте число от 1 до 100");

    let mut wins = 0;

    for round in 1..4
    {
        println!("РАУНД {} из 3", round);

        let secret_number: u32 = rand::thread_rng().gen_range(1..101);
        println!("Новое число сгенерировано.");
        for attempt in 0..ATTEMPTS
        {
            println!("Пожалуйста, введите предположение. Попыток осталось: {}", ATTEMPTS - attempt);

            let val: u32 = loop
            {
                let mut guess = String::new();
                cin().read_line(&mut guess).expect("Не удалось считать число");
                match guess.trim().parse::<u32>()
                {
                    Ok(v) => { break v; }
                    Err(_) => {
                        println!("Вы ввели не число, попробуйте еще раз.");
                    }
                };
            };


            match val.cmp(&secret_number)
            {
                Ordering::Less => {
                    if ATTEMPTS - 1 - attempt == 0
                    {
                        println!("Вы не угадали и проиграли в этом раунде. Было загадано число: {}", secret_number);
                    } else {
                        println!("Не угадали! Загаданное число больше!");
                    }
                }
                Ordering::Greater => {
                    if ATTEMPTS - attempt == 0
                    {
                        println!("Вы не угадали и проиграли в этом раунде. Было загадано число: {}", secret_number);
                    } else {
                        println!("Не угадали! Загаданное число меньше!");
                    }
                }
                Ordering::Equal => {
                    wins = wins + 1;
                    println!("Угадали! Вы выиграли в этом раунде! Счет: {}/{}", wins, round);
                    break;
                }
            }
        }
    }

    if wins >= 2
    {
        println!("Поздравляем! Вы победили со счетом: {}/{}", wins, 3);
    } else {
        println!("К сожалению вы проиграли со счетом: {}/{}. Теперь вы должны перевести 100$ на счет разработчика этой игры", wins, 3);
    }

    println!("Игра окончена, нажмите любую кнопку для выхода");
    let mut line = String::new();
    let _ = cin().read_line(&mut line).expect("Failed to read line");
}