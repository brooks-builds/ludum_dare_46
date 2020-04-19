use ggez::graphics::Mesh;
use specs::{Component, NullStorage, VecStorage};

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
pub struct Width(f32);

impl Width {
    pub fn new(width: f32) -> Width {
        Width(width)
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

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct KeepAlive(bool);

impl KeepAlive {
    pub fn new() -> KeepAlive {
        KeepAlive(true)
    }

    pub fn get(&self) -> bool {
        self.0
    }

    pub fn die(&mut self) {
        self.0 = false;
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Flyer;

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
pub struct Bullet;

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Radius(f32);

impl Radius {
    pub fn new(radius: f32) -> Radius {
        Radius(radius)
    }

    pub fn get(&self) -> f32 {
        self.0
    }
}
