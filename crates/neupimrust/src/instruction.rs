use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Tile {
    instructions: Vec<Instruction>,
}

impl Tile {
    pub fn new() -> Self {
        Tile {
            instructions: Vec::new(),
        }
    }
    pub fn push_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

#[derive(Debug)]
pub struct Instruction {
    parent_tile: Weak<RefCell<Tile>>,
    id: u32,
}
impl Instruction {
    pub fn new(id: u32, parent: Weak<RefCell<Tile>>) -> Self {
        Instruction {
            parent_tile: parent,
            id,
        }
    }
    pub fn set_id(&mut self, id: u32) {
        self.id = id;
    }
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn set_parent_tile(&mut self, parent_tile: Weak<RefCell<Tile>>) {
        self.parent_tile = parent_tile;
    }
    pub fn get_parent_tile(&self) -> Weak<RefCell<Tile>> {
        self.parent_tile.clone()
    }
    pub fn get_parent_tile_strong(&self) -> Rc<RefCell<Tile>> {
        self.parent_tile.upgrade().unwrap()
    }
}
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use super::*;
    #[test]
    fn test_tile() {
        let tile = Rc::new(RefCell::new(Tile::new()));
        let instruction = Instruction::new(0, Rc::downgrade(&tile));
        tile.borrow_mut().push_instruction(instruction);
        println!("{:?}", tile.borrow().instructions);
    }
}
