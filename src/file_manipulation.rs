use std::{
    fs::File,
    io::{BufWriter, Error, Write},
    path::PathBuf,
};

/// Save a file in the provided path with provided content. Creates a new file if it does not
/// exists.
pub fn save(path: &PathBuf, content: &[u8]) -> Result<(), Error> {
    let file = File::create(path)?;
    let mut wbuf = BufWriter::new(file);

    wbuf.write_all(content)
}

/// Returns the language from a file name
pub fn get_language_from_filename<'a>(filename: &String) -> &'a str {
    match filename.split('.').last().unwrap() {
        "c" => "C",
        "cpp" => "C++",
        "cs" => "C#",
        "java" => "Java",
        "js" => "JavaScript",
        "jsx" => "JavaScript",
        "ts" => "TypeScript",
        "tsx" => "TypeScript",
        "py" => "Python",
        "rs" => "Rust",
        "md" => "Markdown",

        _ => "Text",
    }
}
