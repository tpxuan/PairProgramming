use wasm_bindgen::prelude::*;
use std::collections::{VecDeque, HashMap, HashSet};

const GRID_SIZE: i32 = 8;

// 方向定义（dx, dy, dir）
const DIRECTIONS: [(i32, i32, i32); 4] = [
    (0, 1, 0),  // 上 (y)
    (-1, 0, 1), // 左 (-x)
    (0, -1, 2), // 下 (-y)
    (1, 0, 3),  // 右 (+x)
];

#[wasm_bindgen]
pub fn greedy_snake_move_barriers(snake: &[i32], fruit: &[i32], obstacles: &[i32]) -> i32 {
    let mut snake_positions: HashSet<(i32, i32)> = (0..1)
        .map(|i| (snake[i * 2], snake[i * 2 + 1]))
        .collect();
    
    let obstacle_positions: HashSet<(i32, i32)> = (0..12)
        .map(|i| (obstacles[i * 2], obstacles[i * 2 + 1]))
        .collect();

    let head = (snake[0], snake[1]);
    let fruit_pos = (fruit[0], fruit[1]);
    let tail = (snake[6], snake[7]); // 蛇尾的位置

    // 1. 如果果子不可达，直接返回 -1
    if !is_reachable(head, fruit_pos, &snake_positions, &obstacle_positions) {
        return -1;
    }

    // 2. 计算蛇头到果子的最短路径
    if let Some(first_step) = find_shortest_path(head, fruit_pos, &snake_positions, &obstacle_positions) {
        return first_step;
    }

    -1 // 如果没有找到路径，则返回 -1
}

// 使用 BFS 确定果子是否可达
fn is_reachable(
    start: (i32, i32),
    target: (i32, i32),
    snake_positions: &HashSet<(i32, i32)>,
    obstacle_positions: &HashSet<(i32, i32)>,
) -> bool {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(pos) = queue.pop_front() {
        if pos == target {
            return true;
        }

        for &(dx, dy, _) in &DIRECTIONS {
            let next_pos = (pos.0 + dx, pos.1 + dy);

            if next_pos.0 < 1 || next_pos.0 > GRID_SIZE || next_pos.1 < 1 || next_pos.1 > GRID_SIZE {
                continue;
            }

            if snake_positions.contains(&next_pos) || obstacle_positions.contains(&next_pos) {
                continue;
            }

            if !visited.contains(&next_pos) {
                visited.insert(next_pos);
                queue.push_back(next_pos);
            }
        }
    }

    false
}

// 使用 BFS 找到蛇头到果子的最短路径
fn find_shortest_path(
    start: (i32, i32),
    target: (i32, i32),
    snake_positions: &HashSet<(i32, i32)>,
    obstacle_positions: &HashSet<(i32, i32)>,
) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent_map: HashMap<(i32, i32), (i32, i32, i32)> = HashMap::new(); // 记录前驱节点和方向

    queue.push_back(start);
    visited.insert(start);

    while let Some(pos) = queue.pop_front() {
        if pos == target {
            // 反向追踪路径，获取第一步的方向
            let mut current = pos;
            while let Some(&(prev_x, prev_y, dir)) = parent_map.get(&current) {
                if (prev_x, prev_y) == start {
                    return Some(dir);
                }
                current = (prev_x, prev_y);
            }
        }

        for &(dx, dy, dir) in &DIRECTIONS {
            let next_pos = (pos.0 + dx, pos.1 + dy);

            if next_pos.0 < 1 || next_pos.0 > GRID_SIZE || next_pos.1 < 1 || next_pos.1 > GRID_SIZE {
                continue; // 超出边界
            }

            if obstacle_positions.contains(&next_pos) {
                continue; // 撞到障碍物
            }

        
            if snake_positions.contains(&next_pos) {
                continue;  //撞到蛇身
            }

            if !visited.contains(&next_pos) {
                visited.insert(next_pos);
                parent_map.insert(next_pos, (pos.0, pos.1, dir));
                queue.push_back(next_pos);
            }
        }
    }

    None // 没有找到路径
}



#[cfg(test)]
mod tests {
    use super::*;

}
