struct Node{
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
  value: i8
}
impl Node{
  fn new(val: i8) -> Self{
    Node{
      value: val,
      right: None,
      left: None
    }
  }
  fn insert(&mut self, val: i8){
    let new_node = Some(Box::new(Node::new(val)));
    if val < self.value {
      match self.left.as_mut() {
        None => self.left = new_node,
        Some(left) => left.insert(val),
      }
    }
    else {
      match self.right.as_mut() {
        None => self.right = new_node,
        Some(right) => right.insert(val),
      }
    }
  }
}
fn main() {
  let mut binary_tree = Some(Box::new(Node::new(5)));  
}
