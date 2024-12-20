use reqwest::blocking::get;
use reqwest::Error;
use std::env;
use std::fs::File;
use std::io::{self, Write};
use git2::Repository;

fn download_file(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut file = File::create(filename)?;
    let content = response.bytes()?;
    file.write_all(&content)?;
    println!("Файл '{}' успешно загружен.", filename);
    Ok(())
}

fn clone_repository(repo_url: &str, destination: &str) -> Result<(), git2::Error> {
    Repository::clone(repo_url, destination)?;
    println!("Репозиторий '{}' успешно клонирован в '{}'.", repo_url, destination);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Использование: {} <ссылка> <имя файла для сохранения> [clone]", args[0]);
        std::process::exit(1);
    }

    let url = &args[1];
    let filename = &args[2];
    let clone = args.len() > 3 && args[3].to_lowercase() == "clone";

    if clone {
        if let Err(e) = clone_repository(url, filename) {
            eprintln!("Ошибка при клонировании репозитория: {}", e);
        }
    } else {
        if let Err(e) = download_file(url, filename) {
            eprintln!("Ошибка при загрузке файла: {}", e);
        }
    }
}

