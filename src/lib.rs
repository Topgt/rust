use wasm_bindgen::prelude::*;
use rand::Rng;

pub mod my_crate;

pub use crate::my_crate::shape::Square;
pub use crate::my_crate::container::ContainerBox;

#[wasm_bindgen]
pub struct Game {
  container: ContainerBox,
  next_square: Square,
  SHAPE_MATRIXS: Vec<Vec<Vec<u8>>>,
  // #[wasm_bindgen(skip)]
  // pub value: Vec<u32>,
}

#[wasm_bindgen]
impl Game {
  #[wasm_bindgen(constructor)]
  pub fn new(x: usize, y: usize) -> Self {
    Game {
      container: ContainerBox::new(x, y),
      next_square: Square::new(None, None),
      SHAPE_MATRIXS: vec![
        vec![
          vec![0, 0, 1, 0],
          vec![0, 0, 1, 0],
          vec![0, 0, 1, 0],
          vec![0, 0, 1, 0],
        ],
        vec![
          vec![0, 0, 1, 0],
          vec![0, 0, 1, 0],
          vec![0, 0, 1, 1],
          vec![0, 0, 0, 0],
        ],  vec![
          vec![0, 0, 1, 0],
          vec![0, 0, 1, 0],
          vec![0, 1, 1, 0],
          vec![0, 0, 0, 0],
        ],
        vec![
          vec![0, 0, 0, 0],
          vec![0, 1, 1, 0],
          vec![0, 1, 1, 0],
          vec![0, 0, 0, 0],
        ],
        vec![
          vec![0, 0, 0, 0],
          vec![0, 1, 1, 0],
          vec![1, 1, 0, 0],
          vec![0, 0, 0, 0],
        ],
        vec![
          vec![0, 0, 0, 0],
          vec![0, 1, 1, 1],
          vec![0, 0, 1, 0],
          vec![0, 0, 0, 0],
        ],
        vec![
          vec![0, 0, 0, 0],
          vec![0, 1, 1, 0],
          vec![0, 0, 1, 1],
          vec![0, 0, 0, 0],
        ],
      ]
    }
  }

  pub fn add_square(&mut self) {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0..7);
    let mut matrix = self.SHAPE_MATRIXS[random_number].clone();
    if self.container.current_square.is_none() && self.container.is_full == false {
      let square = Square::new(Some(self.next_square.value.clone()), None);
      self.container.add_square(square);
      self.next_square = Square::new(None, Some(matrix));
    }
  }

  pub fn container_is_full(&self) -> bool {
    self.container.is_full
  }

  pub fn move_square_left(&mut self) {
    self.container.move_square_left();
  }

  pub fn move_square_right(&mut self) {
    self.container.move_square_right();
  }

  pub fn move_square_down(&mut self) -> usize {
    self.container.move_square_down();
    match self.container.current_square {
      None => {
        let canel_len = self.container.canel_len.clone();
        self.add_square(); // 添加方块清除 canel_len 的值
        canel_len
      },
      Some(_) => {0}
    }
  }

  pub fn clockwise_rotate_square(&mut self) {
    self.container.clockwise_rotate_square();
  }

  pub fn counterclockwise_rotate_square(&mut self) {
    self.container.counterclockwise_rotate_square();
  }

  pub fn get_container_value(&mut self) -> Vec<u32> {
    self.container.value.clone()
  }

  pub fn current_matrix_value(&mut self) -> Vec<u32> {
    self.container.current_matrix_value()
  }

  pub fn get_square_value(&self) -> Vec<u8> {
    self.next_square.value.clone()
  }

}






