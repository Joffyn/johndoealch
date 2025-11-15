#[derive(Debug, Clone)]
enum Tile
{
    Empty,
    Filled,
    OOB
}
enum TileMapLoadConfig
{
    Box,
}

#[derive(Debug)]
pub struct TileMap
{
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

const DEFAULT_WIDTH: u32 = 8;
const DEFAULT_HEIGHT: u32 = 8;
impl TileMap
{
    fn new(config: TileMapLoadConfig) -> Option<Self>
    {
        let mut tilemap = TileMap
        {
            width: DEFAULT_WIDTH,
            height: DEFAULT_HEIGHT,
            tiles: vec![Tile::Empty; (DEFAULT_WIDTH * DEFAULT_HEIGHT) as usize],

        };
        match config
        {
            TileMapLoadConfig::Box =>
                {
                    for y in 0..tilemap.height
                    {
                        for x in 0..tilemap.width
                        {
                            let edge = x == 0 || y == 0 || x == &tilemap.width - 1 || y == &tilemap.height - 1;
                            let index = tilemap.index(x, y).unwrap();

                            if edge
                            {
                                tilemap.tiles[index] = Tile::Filled;
                            }
                        }
                    }

                }
            _ => return None
        }
        Some(tilemap)
    }
    fn is_empty(&self, x: u32, y: u32) -> bool
    {
        match self.get_tile(x, y)
        {
            Tile::Filled => return true,
            _ => return false,
        }
    }
    fn get_tile(&self, x: u32, y: u32) -> &Tile
    {
        match self.index(x, y)
        {
            Some(i) => &self.tiles[i],
            None => &Tile::OOB
        }
    }
    fn index(&self, x: u32, y: u32) -> Option<usize>
    {
        if !self.in_bounds(x, y)
        {
            return None;
        }
        Some((x * self.height + y) as usize)
    }
    fn in_bounds(&self, x: u32, y: u32) -> bool
    {
        x >= self.width || x < 0 || y >= self.height || y < 0
    }
}

pub fn setup_tilemap()
{

}