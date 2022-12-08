use std::{fs, num::ParseIntError, str::FromStr};

trait Sizeable {
    fn get_size(&self) -> usize;
}

#[derive(Debug)]
struct Directory {
    name: String,
    content: Vec<DirectoryEntry>,
}

#[derive(Debug)]
struct InvalidPath;

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

    fn get_mut<I>(&mut self, mut path: I) -> Result<&mut Directory, InvalidPath>
    where
        I: Iterator<Item = String>,
    {
        match path.next() {
            None => Ok(self),
            Some(name) => self.dir_named_mut(&name).ok_or(InvalidPath)?.get_mut(path),
        }
    }

    fn dir_named_mut(&mut self, name: &str) -> Option<&mut Directory> {
        self.content
            .iter_mut()
            .filter_map(|e| match e {
                DirectoryEntry::Directory(d) => Some(d),
                DirectoryEntry::File(_) => None,
            })
            .find_map(|d| if d.name == name { Some(d) } else { None })
    }
}

impl<'a> IntoIterator for &'a Directory {
    type Item = &'a Directory;
    type IntoIter = std::vec::IntoIter<&'a Directory>;

    fn into_iter(self) -> Self::IntoIter {
        fn append<'a>(dir: &'a Directory, v: &mut Vec<&'a Directory>) {
            v.push(dir);
            for d in dir.content.iter().filter_map(|e| match e {
                DirectoryEntry::Directory(d) => Some(d),
                DirectoryEntry::File(_) => None,
            }) {
                append(d, v)
            }
        }

        let mut result = vec![];
        append(self, &mut result);
        result.into_iter()
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
                .trim()
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

#[derive(Debug)]
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
        let name = sp.next().ok_or(FileParseError)?.trim().to_string();
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

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl Sizeable for File {
    fn get_size(&self) -> usize {
        return self.size;
    }
}

#[derive(Debug)]
enum Process {
    List(List),
    Nav(Nav),
}

#[derive(Debug)]
struct List {
    entries: Vec<DirectoryEntry>,
}

#[derive(Debug)]
enum Nav {
    Root,
    Back,
    Dir(String),
}

#[derive(Debug)]
struct NavParseError;

impl FromStr for Nav {
    type Err = NavParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "/" => Ok(Nav::Root),
            ".." => Ok(Nav::Back),
            _ => Ok(Nav::Dir(s.trim().to_string())),
        }
    }
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
        let mut cmd = lines
            .next()
            .ok_or(ProcessParseError::Empty)?
            .split_whitespace();
        match cmd.next().ok_or(ProcessParseError::Empty)? {
            "ls" => {
                let entries = lines
                    .filter(|l| !l.is_empty())
                    .map(|l| l.parse::<DirectoryEntry>())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Process::List(List { entries }))
            }
            "cd" => {
                let nav = cmd
                    .next()
                    .ok_or(ProcessParseError::Empty)?
                    .parse::<Nav>()
                    .unwrap();
                Ok(Process::Nav(nav))
            }
            _ => Err(Self::Err::CommandNotFound),
        }
    }
}

fn main() {
    let mut root = Directory::new("/".to_string());
    let mut stack = Vec::new();
    for process_res in fs::read_to_string("assets/input.txt")
        .expect("File")
        .split("$ ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<Process>())
    {
        match process_res.expect("Command") {
            Process::List(l) => {
                for entry in l.entries {
                    root.get_mut(stack.iter().cloned())
                        .expect("Path must exist")
                        .add(entry);
                }
            }
            Process::Nav(n) => match n {
                Nav::Root => stack.clear(),
                Nav::Back => {
                    stack.pop().expect("Cannot go back");
                }
                Nav::Dir(d) => stack.push(d),
            },
        }
    }
    let sum = root
        .into_iter()
        .map(|d| d.get_size())
        .filter(|s| *s <= 100000)
        .reduce(|a, b| a + b)
        .expect("Total");
    println!("Total small dirs: {}", sum);
    let min_size = 30000000 - (70000000 - root.get_size());
    let smallest = root
        .into_iter()
        .map(|d| d.get_size())
        .filter(|s| *s > min_size)
        .min()
        .expect("Minimum");
    println!("Minimum size removable: {}", smallest)
}
