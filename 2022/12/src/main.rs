#[derive(Debug)]
struct Tile(u8);

struct InvalidTileChar(char);

impl Tile {
    fn to_height(c: char) -> u8 {
        c as u8 - 'a' as u8 + 1
    }
}

impl TryFrom<char> for Tile {
    type Error = InvalidTileChar;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Tile(Tile::to_height('a') - 1)),
            'E' => Ok(Tile(Tile::to_height('z') + 1)),
            o => {
                if o >= 'a' && o <= 'z' {
                    Ok(Tile(Tile::to_height(o)))
                } else {
                    Err(InvalidTileChar(o))
                }
            }
        }
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Tile>
}

fn main() {
    println!("Hello, world!");
}
