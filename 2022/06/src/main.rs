use std::{num::ParseIntError, str::FromStr};

trait Sizeable {
    fn get_size(&self) -> usize;
}

struct Directory {
    name: String,
    content: Vec<DirectoryEntry>,
}

impl Directory {
    fn new(s: String) -> Directory {
        Directory {
            name: s,
            content: Vec::new(),
        }
    }

    fn add(&mut self, entry: DirectoryEntry) {
        self.content.push(entry);
    }
}

enum DirectoryParseError {
    NotDirectory,
    MissingSection,
}

impl FromStr for Directory {
    type Err = DirectoryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split(" ");
        if sp
            .next()
            .ok_or(DirectoryParseError::MissingSection)?
            .eq("dir")
        {
            let name = sp
                .next()
                .ok_or(DirectoryParseError::MissingSection)?
                .to_string();
            Ok(Directory::new(name))
        } else {
            Err(DirectoryParseError::NotDirectory)
        }
    }
}

impl Sizeable for Directory {
    fn get_size(&self) -> usize {
        self.content
            .iter()
            .map(|e| e.get_size())
            .reduce(|a, b| a + b)
            .unwrap_or(0)
    }
}

enum DirectoryEntry {
    Directory(Directory),
    File(File),
}

#[derive(Debug)]
struct InvalidDirectoryEntry(String);

impl FromStr for DirectoryEntry {
    type Err = InvalidDirectoryEntry;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<Directory>() {
            Ok(d) => Ok(Self::from(d)),
            Err(DirectoryParseError::NotDirectory) => Ok(Self::from(
                s.parse::<File>()
                    .map(|f| Self::from(f))
                    .map_err(|_| InvalidDirectoryEntry(s.to_string()))?,
            )),
            Err(_) => Err(InvalidDirectoryEntry(s.to_string())),
        }
    }
}

impl From<Directory> for DirectoryEntry {
    fn from(d: Directory) -> Self {
        DirectoryEntry::Directory(d)
    }
}

impl From<File> for DirectoryEntry {
    fn from(f: File) -> Self {
        DirectoryEntry::File(f)
    }
}

struct FileParseError;

impl From<ParseIntError> for FileParseError {
    fn from(_: ParseIntError) -> Self {
        Self
    }
}

impl FromStr for File {
    type Err = FileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sp = s.split_whitespace();
        let size = sp.next().ok_or(FileParseError)?.parse::<usize>()?;
        let name = sp.next().ok_or(FileParseError)?.to_string();
        Ok(File { name, size })
    }
}

impl Sizeable for DirectoryEntry {
    fn get_size(&self) -> usize {
        match self {
            DirectoryEntry::Directory(d) => d.get_size(),
            DirectoryEntry::File(f) => f.get_size(),
        }
    }
}

struct File {
    name: String,
    size: usize,
}

impl Sizeable for File {
    fn get_size(&self) -> usize {
        return self.size;
    }
}

enum Process {
    List(List),
    Nav(),
}

struct List {
    entries: Vec<DirectoryEntry>,
}

enum Nav {
    Root,
    Back,
    Dir(String),
}

#[derive(Debug)]
enum ProcessParseError {
    Empty,
    CommandNotFound,
    ParseListEntry(InvalidDirectoryEntry),
}

impl From<InvalidDirectoryEntry> for ProcessParseError {
    fn from(err: InvalidDirectoryEntry) -> Self {
        Self::ParseListEntry(err)
    }
}

impl FromStr for Process {
    type Err = ProcessParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n");
        let cmd = lines
            .next()
            .ok_or(ProcessParseError::Empty)?
            .split_whitespace();
        match cmd.next().ok_or(ProcessParseError::Empty)? {
            "ls" => {
                let entries = lines
                    .map(|l| l.parse::<DirectoryEntry>())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Process::List(List { entries }))
            }
            _ => Err(Self::Err::CommandNotFound),
        }
    }
}

fn main() {
    let root = Directory::new("/".to_string());
}
