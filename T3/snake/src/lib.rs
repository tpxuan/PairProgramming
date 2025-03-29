use wasm_bindgen::prelude::*;
use std::collections::{HashSet, VecDeque, HashMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[wasm_bindgen]
pub fn greedy_snake_step(
    n: i32,
    snake: Vec<i32>,
    snake_num: i32,
    other_snakes: Vec<i32>,
    food_num: i32,
    foods: Vec<i32>,
    round: i32,
) -> i32 {
    // **解析自己的蛇**
    let mut my_snake = Vec::new();
    for i in (0..snake.len()).step_by(2) {
        my_snake.push(Position { x: snake[i], y: snake[i + 1] });
    }
    let head = my_snake[0];

    // **优化障碍物计算**
    let mut obstacles = HashSet::new();
    let mut obstaclesPre = HashSet::new();

    // **仅将自己蛇的第二节身子作为障碍**
    if my_snake.len() > 1 {
        obstacles.insert(my_snake[1]);
    }

    // **处理其他蛇**
    for i in 0..snake_num as usize {
        let base_idx = i * 8;

        // **前三节身体作为障碍**
        for j in 0..3 {
            let idx = base_idx + j * 2;
            if idx + 1 < other_snakes.len() {
                obstacles.insert(Position { x: other_snakes[idx], y: other_snakes[idx + 1] });
            }
        }

        // **蛇头的周围四格也作为障碍**
        if base_idx + 1 < other_snakes.len() {
            let snake_head = Position { x: other_snakes[base_idx], y: other_snakes[base_idx + 1] };
            for &(dx, dy) in &[(0, 1), (-1, 0), (0, -1), (1, 0)] {
                let adjacent = Position { x: snake_head.x + dx, y: snake_head.y + dy };
                if adjacent.x >= 1 && adjacent.x <= n && adjacent.y >= 1 && adjacent.y <= n {
                    obstacles.insert(adjacent);
                }
            }
        }
    }

    // **仅将自己蛇的第二节身子作为障碍**
    if my_snake.len() > 1 {
        obstaclesPre.insert(my_snake[1]);
    }

    // **处理其他蛇**
    for i in 0..snake_num as usize {
        let base_idx = i * 8;

        // **前三节身体作为障碍**
        for j in 0..3 {
            let idx = base_idx + j * 2;
            if idx + 1 < other_snakes.len() {
                obstaclesPre.insert(Position { x: other_snakes[idx], y: other_snakes[idx + 1] });
            }
        }
    }

    // **解析食物位置**
    let food_positions: HashSet<Position> = foods.chunks(2).map(|c| Position { x: c[0], y: c[1] }).collect();

    // **使用 BFS 搜索最近的食物**
    if let Some(path) = bfs_search(n, head, &food_positions, &obstacles) {
        let next_move = path[1];  // 取第二个位置作为下一步
        return match (next_move.x - head.x, next_move.y - head.y) {
            (0, 1) => 0,   // 上
            (-1, 0) => 1,  // 左
            (0, -1) => 2,  // 下
            (1, 0) => 3,   // 右
            _ => 0,
        };
    }

    // **生存优先策略**
    for &(dx, dy, dir) in &[(0, 1, 0), (-1, 0, 1), (0, -1, 2), (1, 0, 3)] {
        let new_pos = Position { x: head.x + dx, y: head.y + dy };
        if new_pos.x >= 1 && new_pos.x <= n && new_pos.y >= 1 && new_pos.y <= n && !obstaclesPre.contains(&new_pos) {
            return dir;
        }
    }

    0 // 默认向上
}

// **BFS 搜索最近食物**
fn bfs_search(
    n: i32,
    start: Position,
    goals: &HashSet<Position>,
    obstacles: &HashSet<Position>,
) -> Option<Vec<Position>> {
    let mut queue = VecDeque::new();
    let mut came_from = HashMap::new();
    let mut visited = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if goals.contains(&current) {
            return Some(reconstruct_path(came_from, current));
        }

        for &(dx, dy) in &[(0, 1), (-1, 0), (0, -1), (1, 0)] {
            let neighbor = Position { x: current.x + dx, y: current.y + dy };

            if neighbor.x < 1 || neighbor.x > n || neighbor.y < 1 || neighbor.y > n {
                continue; // 超出棋盘边界
            }
            if obstacles.contains(&neighbor) || visited.contains(&neighbor) {
                continue; // 遇到障碍物或已经访问过
            }

            queue.push_back(neighbor);
            visited.insert(neighbor);
            came_from.insert(neighbor, current);
        }
    }

    None // 没有找到路径
}

// **重建路径**
fn reconstruct_path(mut came_from: HashMap<Position, Position>, mut current: Position) -> Vec<Position> {
    let mut path = vec![current];
    while let Some(&prev) = came_from.get(&current) {
        path.push(prev);
        current = prev;
    }
    path.reverse();
    path
}
