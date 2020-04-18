use ggez::graphics::Mesh;
use specs::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct HasGravity;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Floor;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Height(f32);

impl Height {
    pub fn new(height: f32) -> Height {
        Height(height)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ObjectMesh(Mesh);

impl ObjectMesh {
    pub fn new(mesh: Mesh) -> ObjectMesh {
        ObjectMesh(mesh)
    }

    pub fn get(&self) -> &Mesh {
        &self.0
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Acceleration {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Drag(f32);

impl Drag {
    pub fn new(drag: f32) -> Drag {
        Drag(drag)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct OnGround(bool);

impl OnGround {
    pub fn new() -> OnGround {
        OnGround(false)
    }

    pub fn get(&self) -> bool {
        self.0
    }

    pub fn set(&mut self, new_value: bool) {
        self.0 = new_value;
    }
}
