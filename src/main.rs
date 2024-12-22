use std::process::{Command, Stdio};

fn main() {
    // Получаем аргументы командной строки
    let args: Vec<String> = std::env::args().collect();

    // Проверяем, что была передана хотя бы одна команда
    if args.len() < 2 {
        println!("Использование: {} <команда_для_выполнения_с_правами_администратора>", args[0]);
        return;
    }

    // Команда, которую мы хотим выполнить с привилегиями администратора
    let command = &args[1];

    // Формируем команду для PowerShell
    let full_command = format!("Start-Process {} -ArgumentList '-NoProfile -ExecutionPolicy Bypass -Command {}' -Verb RunAs", command,command);

    // Запускаем команду в PowerShell
    let output = Command::new("powershell")
        .arg("-Command")
        .arg(full_command)
        .output()
        .expect("Не удалось выполнить команду");

    // Проверяем результат выполнения
    if output.status.success() {
        println!("Команда выполнена успешно.");
    } else {
        eprintln!("Ошибка выполнения команды: {}", String::from_utf8_lossy(&output.stderr));
    }
}
