use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{Read, Result},
    path::{Path, PathBuf},
    process::ExitCode,
};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let dir = args.get(1).map(String::as_str).unwrap_or(".");
    let root = Path::new(dir);

    if !root.is_dir() {
        eprintln!("'{}' — не директория", dir);
        return ExitCode::from(1);
    }

    let groups = match find_duplicates(root) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("ошибка обхода: {e}");
            return ExitCode::from(2);
        }
    };

    if groups.is_empty() {
        println!("дубликаты не найдены");
        return ExitCode::SUCCESS;
    }

    println!("найдено групп дубликатов: {}", groups.len());
    for (i, group) in groups.iter().enumerate() {
        println!("\n#{}", i + 1);
        for p in group {
            println!("  {}", p.display());
        }
    }
    ExitCode::SUCCESS
}

/// Итеративный обход: возвращает пути всех обычных файлов
/// внутри `root`. Не уходит в рекурсию, не читает содержимое файлов.
fn collect_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Пропускаю {}: {}", dir.display(), err);
                continue;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            if meta.is_dir() {
                stack.push(path);
            } else if meta.is_file() {
                files.push(path);
            }
            // Блочные устройства, FIFO, сокеты — игнорируем.
        }
    }

    Ok(files)
}

/// Группирует пути по размеру файла. Группы из одного файла
/// сразу выкидываем — у них не может быть дубликата.
fn group_by_size(files: Vec<PathBuf>) -> HashMap<u64, Vec<PathBuf>> {
    let mut by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for path in files {
        if let Ok(meta) = fs::metadata(&path) {
            by_size.entry(meta.len()).or_default().push(path);
        }
    }

    by_size.retain(|_, group| group.len() > 1);
    by_size
}

const HEAD_BYTES: usize = 64 * 1024; // 64 КБ

/// Хеш первых HEAD_BYTES байт файла. Файл целиком в память
/// не загружается — читаем ровно один блок.
fn partial_hash(path: &Path) -> Result<[u8; 32]> {
    let mut file = File::open(path)?;
    let mut buf = vec![0u8; HEAD_BYTES];
    let n = file.read(&mut buf)?;
    let mut hasher = Sha256::new();
    hasher.update(&buf[..n]);
    Ok(hasher.finalize().into())
}

fn full_hash(path: &Path) -> Result<[u8; 32]> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = [0u8; 64 * 1024];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().into())
}

type Hash = [u8; 32];

fn find_duplicates(root: &Path) -> std::io::Result<Vec<Vec<PathBuf>>> {
    // 1) собрать все файлы
    let files = collect_files(root)?;

    // 2) сгруппировать по размеру
    let by_size = group_by_size(files);

    // 3) внутри каждой size-группы — частичный хеш
    let mut by_partial: HashMap<(u64, Hash), Vec<PathBuf>> = HashMap::new();
    for (size, group) in by_size {
        for path in group {
            if let Ok(h) = partial_hash(&path) {
                by_partial.entry((size, h)).or_default().push(path);
            }
        }
    }
    by_partial.retain(|_, g| g.len() > 1);

    // 4) внутри каждой (size, partial)-группы — полный хеш
    let mut result: Vec<Vec<PathBuf>> = Vec::new();
    for (_key, group) in by_partial {
        let mut by_full: HashMap<Hash, Vec<PathBuf>> = HashMap::new();
        for path in group {
            if let Ok(h) = full_hash(&path) {
                by_full.entry(h).or_default().push(path);
            }
        }
        for (_h, g) in by_full {
            if g.len() > 1 {
                result.push(g);
            }
        }
    }

    Ok(result)
}
