use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greedy_snake_move(snake: &[i32], fruit: &[i32]) -> i32 {
    // 确保输入数组长度正确
    if snake.len() != 8 || fruit.len() != 2 {
        return -1;
    }

    let (head_x, head_y) = (snake[0], snake[1]);
    let (tail_x, tail_y) = (snake[6], snake[7]); // 蛇尾坐标
    let (fruit_x, fruit_y) = (fruit[0], fruit[1]);

    // 方向对应的坐标变化: 上 (0,1)，左 (-1,0)，下 (0,-1)，右 (1,0)
    let directions = [(0, 1), (-1, 0), (0, -1), (1, 0)];
    let mut best_move = -1;

    // 避免蛇撞到自己或墙
    let mut is_safe = [true; 4];

    for (i, (dx, dy)) in directions.iter().enumerate() {
        let new_x = head_x + dx;
        let new_y = head_y + dy;

        // 检查是否撞墙
        if new_x < 1 || new_x > 8 || new_y < 1 || new_y > 8 {
            is_safe[i] = false;
            continue;
        }

        // 检查是否撞到自己的身体
        if new_x == snake[2] && new_y == snake[3] {
            is_safe[i] = false;
            continue;
        }
    }

    // **策略 1：优先选择能安全前往果子的方向**
    let mut min_distance = i32::MAX;
    for (i, &(dx, dy)) in directions.iter().enumerate() {
        if !is_safe[i] {
            continue;
        }

        let new_x = head_x + dx;
        let new_y = head_y + dy;
        let distance = (new_x - fruit_x).abs() + (new_y - fruit_y).abs();

        if distance < min_distance {
            min_distance = distance;
            best_move = i as i32;
        }
    }

    // **策略 2：如果所有方向都不安全（已被围住），返回上（-1）**
    if best_move == -1 {
        return -1;
    }

    best_move
}


#[cfg(test)]
mod tests {
    use super::*;

}
