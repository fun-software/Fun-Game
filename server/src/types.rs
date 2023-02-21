use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Entity {
    pub id: u64,
    pub pos: Position,
}

impl Position {
    pub fn to_json(&self) -> String {
        format!("{{\"x\":{},\"y\":{}, \"z\":{}}}", self.x, self.y, self.z)
    }
}

impl Entity {
    pub fn to_json(&self) -> String {
        format!(
            "{{\"position\":{{\"x\":{},\"y\":{}, \"z\":{}}}, \"id\":{}}}",
            self.pos.x, self.pos.y, self.pos.z, self.id
        )
    }
}

pub type Entities = HashMap<u64, Entity>;
