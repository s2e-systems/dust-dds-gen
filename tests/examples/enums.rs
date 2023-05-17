#[derive(Debug, serde::Deserialize, serde::Serialize, dust_dds::DdsType)]
pub enum Suits {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}
#[derive(Debug, serde::Deserialize, serde::Serialize, dust_dds::DdsType)]
pub enum Direction {
    North,
    East,
    South,
    West,
}