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
    // 测试蛇头向四个基本方向移动的正确性
    #[test]
    fn test_move_up() {
        // 蛇头在(4,4)，果实在(4,5)
        let snake = [4,4, 4,3, 5,3, 5,2];
        let fruit = [4,5];
        assert_eq!(greedy_snake_move(&snake, &fruit), 0); // 上
    }
    
    #[test]
    fn test_move_left() {
        // 蛇头在(4,4)，果实在(3,4)
        let snake = [4,4, 5,4, 5,5, 6,5];
        let fruit = [3,4];
        assert_eq!(greedy_snake_move(&snake, &fruit), 1); // 左
    }
    
    #[test]
    fn test_move_down() {
        // 蛇头在(4,4)，果实在(4,3)
        let snake = [4,4, 4,5, 3,5, 3,6];
        let fruit = [4,3];
        assert_eq!(greedy_snake_move(&snake, &fruit), 2); // 下
    }
    
    #[test]
    fn test_move_right() {
        // 蛇头在(4,4)，果实在(5,4)
        let snake = [4,4, 3,4, 3,3, 3,2];
        let fruit = [5,4];
        assert_eq!(greedy_snake_move(&snake, &fruit), 3); // 右
    }

    // 测试函数对错误输入的处理
    #[test]
    fn test_invalid_snake_length() {
        // 蛇身坐标数组长度错误
        let snake = [1,1, 1,2]; // 只有2节
        let fruit = [2,2];
        assert_eq!(greedy_snake_move(&snake, &fruit), -1);
    }
    
    #[test]
    fn test_invalid_fruit_length() {
        // 果实坐标数组长度错误
        let snake = [1,1, 1,2, 2,2, 2,1];
        let fruit = [2]; // 只有1个坐标
        assert_eq!(greedy_snake_move(&snake, &fruit), -1);
    }

    // 测试蛇避开自身身体
    #[test]
    fn test_avoid_body_up() {
        // 向上会撞到身体，应选择其他方向
        let snake = [4,4, 4,5, 3,5, 3,6];
        let fruit = [4,5]; // 果实在身体位置
        let result = greedy_snake_move(&snake, &fruit);
        assert_ne!(result, 0); // 不应向上
    }

    // 测试哈曼顿距离最短
    #[test]
    fn test_equal_manhattan_choices() {
        let snake = [4,4, 4,5, 3,5, 3,6];
        let fruit = [2,4]; // 曼哈顿距离: 左(2), 下(3), 上(5), 右(6)
        let result = greedy_snake_move(&snake, &fruit);
        assert_eq!(result, 1); // 应选择左(距离更短)
    }

    // 模拟200回合内是否能吃到果子
    #[test]
    fn test_round_limit() {
        let mut snake = [4,4, 4,3, 5,3, 5,2]; // 初始位置
        let mut fruit = [1,1]; // 初始果子位置

        for round in 1..=200 {
            let direction = greedy_snake_move(&snake, &fruit);
            
            // 更新蛇的位置
            let (head_x, head_y) = (snake[0], snake[1]);
            let new_head = match direction {
                0 => (head_x, head_y + 1), // 上
                1 => (head_x - 1, head_y), // 左
                2 => (head_x, head_y - 1), // 下
                3 => (head_x + 1, head_y), // 右
                _ => panic!("Invalid direction"),
            };
            
            // 检查是否撞墙
            if new_head.0 < 1 || new_head.0 > 8 || new_head.1 < 1 || new_head.1 > 8 {
                panic!("Hit wall at round {}", round);
            }
            
            // 检查是否吃到果子
            if new_head.0 == fruit[0] && new_head.1 == fruit[1] {
                return; // 测试通过
            }
            
            // 更新蛇身
            let mut new_snake = [0; 8];
            new_snake[0] = new_head.0;
            new_snake[1] = new_head.1;
            for i in 0..6 {
                new_snake[i+2] = snake[i];
            }
            snake = new_snake;
        }
        
        panic!("Failed to eat fruit within 200 rounds");
    }

    // 时间限制测试
    #[test]
    fn test_time_limit() {
        let snake = [4,4, 4,3, 5,3, 5,2];
        let fruit = [1,1];
        
        let start_time = std::time::Instant::now();
        let mut iterations = 0;
        
        while start_time.elapsed().as_millis() < 1000 {
            let _ = greedy_snake_move(&snake, &fruit);
            iterations += 1;
        }
        
        assert!(iterations > 1000, "Performance too slow: {} iterations", iterations);
    }
}
