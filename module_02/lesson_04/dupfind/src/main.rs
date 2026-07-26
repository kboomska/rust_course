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
    let config = parse_config(&args);
    let dir = Path::new(&config.root);

    if !dir.is_dir() {
        eprintln!("'{}' — не директория", &config.root);
        return ExitCode::from(1);
    }

    let groups = match find_duplicates(&config) {
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

    let mut total_size: u64 = 0;
    for (i, group) in groups.iter().enumerate() {
        let dup_count = group.files.len();
        let dup_size = group.size * (dup_count as u64 - 1);
        total_size += dup_size;

        println!("\n#{} Дубликатов: {}\n", i + 1, dup_count);
        for p in &group.files {
            println!("    {}", p.display());
        }
        println!("\n    Размер файла: {}", format_size(group.size));
        println!(
            "    Объем дублированных данных: {}\n",
            format_size(dup_size)
        );
    }
    println!("Найдено групп дубликатов: {}", groups.len());
    println!("Можно сэкономить места: {}", format_size(total_size));
    ExitCode::SUCCESS
}

struct Config {
    root: String,
    recursive: bool,
    min_size: Option<u64>,
    extensions: Option<Vec<String>>,
}

struct Group {
    size: u64,
    files: Vec<PathBuf>,
}

fn parse_config(args: &[String]) -> Config {
    let mut root = String::from(".");
    let mut recursive = true;
    let mut min_size = None;
    let mut extensions = None;

    for arg in args.iter().skip(1) {
        match arg.as_str() {
            "--no-recursive" => {
                recursive = false;
            }
            _ if arg.starts_with("--min-size=") => {
                let value = &arg["--min-size=".len()..];
                min_size = value.parse::<u64>().ok();
            }
            _ if arg.starts_with("--ext=") => {
                let ext = &arg["--ext=".len()..];
                if !ext.is_empty() {
                    let ext = ext.split(',').map(|e| e.to_string()).collect();
                    extensions = Some(ext);
                }
            }
            _ if !arg.starts_with('-') => {
                root = arg.to_string();
            }
            _ => eprintln!("Неизвестный аргумент: {arg}"),
        }
    }

    Config {
        root,
        recursive,
        min_size,
        extensions,
    }
}

/// Итеративный обход: возвращает пути всех обычных файлов
/// внутри `root`. Не уходит в рекурсию, не читает содержимое файлов.
fn collect_files(config: &Config) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let path = Path::new(&config.root);
    let mut stack: Vec<PathBuf> = vec![path.to_path_buf()];

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

            if meta.is_dir() && config.recursive {
                stack.push(path);
            } else if meta.is_file() && check_extension(config, &path) {
                files.push(path);
            }
            // Блочные устройства, FIFO, сокеты — игнорируем.
        }
    }

    Ok(files)
}

/// Группирует пути по размеру файла. Группы из одного файла
/// сразу выкидываем — у них не может быть дубликата.
fn group_by_size(
    files: Vec<PathBuf>,
    config: &Config,
) -> HashMap<u64, Vec<PathBuf>> {
    let mut by_size: HashMap<u64, Vec<PathBuf>> = HashMap::new();

    for path in files {
        if let Ok(meta) = fs::metadata(&path) {
            let size = meta.len();

            if let Some(min_size) = config.min_size
                && size < min_size
            {
                continue;
            } else {
                by_size.entry(size).or_default().push(path);
            }
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

fn find_duplicates(config: &Config) -> std::io::Result<Vec<Group>> {
    // 1) собрать все файлы
    let files = collect_files(config)?;

    // 2) сгруппировать по размеру
    let by_size = group_by_size(files, config);

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
    let mut result: Vec<Group> = Vec::new();
    for ((size, _), group) in by_partial {
        let mut by_full: HashMap<Hash, Vec<PathBuf>> = HashMap::new();
        for path in group {
            if let Ok(h) = full_hash(&path) {
                by_full.entry(h).or_default().push(path);
            }
        }
        for (_h, files) in by_full {
            if files.len() > 1 {
                result.push(Group { size, files });
            }
        }
    }

    Ok(result)
}

fn format_size(bytes: u64) -> String {
    let size_suffix = ["Б", "кБ", "МБ", "ГБ", "ТБ", "ПБ"];
    let mut exponent: u64 = 1;
    let mut suffix_index = 0;

    while bytes / exponent >= 1000 && suffix_index < size_suffix.len() - 1 {
        exponent *= 1000;
        suffix_index += 1;
    }

    let whole = bytes / exponent;
    let remainder = (bytes % exponent) * 100 / exponent;

    format!("{}.{:02} {}", whole, remainder, size_suffix[suffix_index])
}

fn check_extension(config: &Config, path: &Path) -> bool {
    if let Some(extensions) = &config.extensions {
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            return extensions.iter().any(|e| e.eq_ignore_ascii_case(ext));
        } else {
            return false;
        }
    }

    true
}
