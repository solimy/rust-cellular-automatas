use super::{Cell, Neighbors};

pub fn tick(cell: &mut Cell, neighbors: &Neighbors) {
    let alive_neighbors = [
        neighbors.up_left,
        neighbors.up,
        neighbors.up_right,
        neighbors.left,
        neighbors.right,
        neighbors.down_left,
        neighbors.down,
        neighbors.down_right,
    ]
    .iter()
    .filter(|c| c.is_some())
    .filter(|c| c.unwrap().is_alive)
    .count() as u8;
    cell.is_alive = match (cell.is_alive, alive_neighbors) {
        (true, 2) | (true, 3) => true,
        (false, 3) | (false, 6) => true,
        _ => false,
    }
}
