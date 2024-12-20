import requests
import sys
import os
from git import Repo

def download_file(url, filename):
    try:
        response = requests.get(url)
        response.raise_for_status()  # Проверка на ошибки HTTP

        with open(filename, 'wb') as file:
            file.write(response.content)
        print(f"Файл '{filename}' успешно загружен.")
    except requests.exceptions.RequestException as e:
        print(f"Ошибка при загрузке файла: {e}")

def get_filename_from_url(url):
    return os.path.basename(url)

def clone_repository(repo_url, destination):
    try:
        Repo.clone_from(repo_url, destination)
        print(f"Репозиторий '{repo_url}' успешно клонирован в '{destination}'.")
    except Exception as e:
        print(f"Ошибка при клонировании репозитория: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Использование: python download.py <ссылка> <имя файла для сохранения> [clone]")
        sys.exit(1)

    url = sys.argv[1]
    filename = sys.argv[2]
    clone = len(sys.argv) > 3 and sys.argv[3].lower() == 'clone'

    if clone:
        clone_repository(url, filename)
    else:
        if not filename:
            filename = get_filename_from_url(url)
        download_file(url, filename)

