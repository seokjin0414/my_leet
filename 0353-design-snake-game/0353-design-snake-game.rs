use std::collections::VecDeque;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Pos {
  y: i32,
  x: i32,
}

struct SnakeGame {
  Y: i32,
  X: i32,
  snake_deque: VecDeque<Pos>,
  snake_set: HashSet<Pos>,
  food: Vec<Pos>,
  food_pos: usize,
}

impl SnakeGame {

  fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
    let mut food_data = Vec::with_capacity(food.len() + 1);
    for el in food {
      food_data.push(Pos{y: el[0], x: el[1]});
    }
    food_data.push(Pos{y: -1, x: -1});

    return SnakeGame{
      Y: height,
      X: width,
      snake_deque: VecDeque::from([Pos{y: 0, x: 0}]),
      snake_set: HashSet::from([Pos{y: 0, x: 0}]),
      food: food_data,
      food_pos: 0,
    }
  }
  
  fn make_a_move(&mut self, direction: String) -> i32 {
    let p = self.snake_deque.front().unwrap_or(&Pos{y:-1, x:-1});
    let (mut y, mut x) = (p.y, p.x);
    if direction == "U" {
      y -= 1;
    } else if direction == "R" {
      x += 1;
    } else if direction == "D" {
      y += 1;
    } else {
      x -= 1;
    }

    if y < 0 || x < 0 || y == self.Y || x == self.X {
      return -1;
    }

    self.snake_deque.push_front(Pos{y: y, x: x});
    if y == self.food[self.food_pos].y && x == self.food[self.food_pos].x {
      self.food_pos += 1;
    } else {
      self.snake_set.remove(&self.snake_deque.pop_back().unwrap_or(Pos{y:-1, x:-1}));
    }

    if self.snake_set.contains(&Pos{y: y, x: x}) {
      return -1;
    }

    self.snake_set.insert(Pos{y: y, x: x});
    return (self.snake_deque.len() - 1) as i32;
  }
}