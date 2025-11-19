#[derive(Debug, Clone)]
pub enum Tile
{
    Empty,
    Filled,
    OOB
}
pub enum TileMapLoadConfig
{
    Box,
}

#[derive(Debug)]
pub struct Pos
{
    pub x: f32,
    pub y: f32
}

#[derive(Debug)]
pub struct TileMap
{
    tiles: Vec<Tile>,
    width: u32,
    height: u32,
}

const DEFAULT_WIDTH: u32 = 32;
const DEFAULT_HEIGHT: u32 = 32;
impl TileMap
{
    //pub fn get_visible_tiles(&self, cam_pos: &Pos,  viewport_width: u32, viewport_height: u32) -> Vec<Tile>
    //{
    //    let mut visible_tiles = vec![Tile::Empty; (viewport_width * viewport_height * 2) as usize];
    //    let bot = cam_pos.y.into() - viewport_height;
    //    let left = cam_pos.x.into() - viewport_width;
    //    for y in bot..viewport_height * 2
    //    {
    //        for x in left..viewport_width * 2
    //        {
    //            //visible_tiles[self.get_tile(x, y)];
    //        }
    //    }
    //    visible_tiles
    //}
    pub fn new(config: TileMapLoadConfig) -> Option<Self>
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
                            if edge
                            {
                                match tilemap.index(x, y)
                                {
                                    Some(i) => tilemap.tiles[i] = Tile::Filled,
                                    None => eprintln!("X: {}, Y: {}, was OOB", x, y),
                                }
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
        !(x >= self.width || x < 0 || y >= self.height || y < 0)
    }
}
