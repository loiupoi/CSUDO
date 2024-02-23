use std::process::{Command, Stdio};

fn main() {
    // Получаем аргументы командной строки
    let args: Vec<String> = std::env::args().collect();

    // Проверяем, что была передана хотя бы одна команда
    if args.len() < 2 {
        println!("Usage: {} <command_to_execute_with_admin_privileges>", args[0]);
        return;
    }

    // Команда, которую мы хотим выполнить с привилегиями администратора
    let command = &args[1];

    // Запускаем команду cmd.exe с аргументами /C и нашей командой
    let mut child = Command::new("cmd.exe")
        .args(&["/C", command])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::inherit())
        .spawn()
        .expect("Failed to execute command");

    // Ожидаем завершения выполнения команды
    let _ = child.wait().expect("Failed to wait on child process");
}
