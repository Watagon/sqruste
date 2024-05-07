use std::io::{Read, Write};

#[derive(Debug, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

pub mod size {
    pub const ID: usize = 32;
    pub const USERNAME: usize = 32;
    pub const EMAIL: usize = 255;
    pub const ALL: usize = ID + USERNAME + EMAIL;
}

pub mod offset {
    use super::size;
    pub const ID: usize = 0;
    pub const USERNAME: usize = size::ID;
    pub const EMAIL: usize = USERNAME + size::USERNAME;
}

pub fn serialize_row<W: Write>(row: &User, dst: &mut W) {
    // _ = dst.write()
    todo!();
}

pub fn deserialize_row<R: Read>(src: &mut R) -> User {
    todo!()
}
