use super::{Cell, Neighbors};

pub fn tick(cell: &mut Cell, neighbors: &Neighbors, stack: bool) {
    cell.is_protected = if let Some(down) = neighbors.down {
        down.is_protected && cell.is_alive && stack
    } else {
        cell.is_alive && stack
    };

    cell.is_alive = if let Some(up) = neighbors.up {
        (cell.is_protected && cell.is_alive) || (!cell.is_protected && up.is_alive)
    } else {
        cell.is_protected && cell.is_alive
    };

    cell.get_older();

    // println!("alive({}), protected({})", cell.is_alive, cell.is_protected)
}

pub fn populate(y: usize, cell: &mut Cell, p: f32) {
    cell.is_alive = if y == 0 {
        rand::random::<f32>() < p
    } else {
        cell.is_alive
    }
}
