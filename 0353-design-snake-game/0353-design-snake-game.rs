use std::collections::{HashMap, HashSet, VecDeque};

struct SnakeGame {
    width: i32,
    height: i32,
    food: Vec<(i32, i32)>,
    food_index: usize,
    directions: HashMap<char, (i32, i32)>,
    snake_set: HashSet<(i32, i32)>,
    snake: VecDeque<(i32, i32)>,
}

impl SnakeGame {

    fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
       let mut directions = HashMap::new();
        directions.insert('L', (0, -1));
        directions.insert('R', (0, 1));
        directions.insert('U', (-1, 0));
        directions.insert('D', (1, 0));

        let mut snake = VecDeque::new();
        snake.push_front((0, 0));

        let mut snake_set = HashSet::new();
        snake_set.insert((0, 0));

        SnakeGame{
            width,
            height,
            food: food.into_iter().map(|f| (f[0], f[1])).collect(),
            food_index: 0,
            directions,
            snake_set,
            snake,
        }
    }
    
    fn make_a_move(&mut self, direction: String) -> i32 {
        let direction_char = direction.chars().next().unwrap();
        let (dx, dy) = self.directions[&direction_char];
        let (head_x, head_y) = self.snake.front().unwrap();
        let new_head = (head_x + dx, head_y + dy);

        let out_of_bounds = new_head.0 < 0 || new_head.0 >= self.height || new_head.1 < 0 || new_head.1 >= self.width;
        let bites_self = self.snake_set.contains(&new_head) && * self.snake.back().unwrap() != new_head;

        if bites_self || out_of_bounds {
            return -1;
        }

        let eat_food = self.food_index < self.food.len() && self.food[self.food_index] == new_head;

        if eat_food {
            self.food_index += 1;
        } else {
            let tail = self.snake.pop_back().unwrap();
            self.snake_set.remove(&tail);
        }

        self.snake.push_front(new_head);
        self.snake_set.insert(new_head);

        (self.snake.len() as i32) - 1
    }
}