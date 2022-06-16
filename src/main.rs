use crate::esen_func::rand_range;

mod esen_func;

struct Settlement
{
    army_rate: u8,
    population: u32,
    resource_makers: u32,
}

impl Settlement
{
    fn new() -> Self {
        Self {
            army_rate: rand_range(10, 30) as u8,
            population: rand_range(300, 1000) as u32,
            resource_makers: rand_range(1, 10) as u32
        }
    }
}

struct World
{
    max_land: u32,
    settlements: Vec<Settlement>
}

impl World
{
    fn new() -> Self {
        Self {
            max_land: rand_range(10000, 10000) as u32,
            settlements: vec![]
        }
    }
}

fn main() {
}
