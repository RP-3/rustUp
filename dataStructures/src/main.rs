enum Node {
  Cons(int, Box<Node>), //Cons reserved word? //has type 'Node'
  Nil
}

impl Node {

  fn new()->Node{
    Nil //Nil should have type 'Node'. Replace w/ Null?
  }

  fn append(self, elem: int)->Node{
    Cons(elem, box self)
  }

  fn stringify(&self)->String{
    match *self {
      Cons(head, ref tail) => {
        format!("{}, [ ] -> {}", head, tail.stringify())
      },
      Nil => {
        format!("Nil")
      }
    }
  }

}

fn main() {
  let mut list = Node::new();

  list = list.append(1);
  list = list.append(2);
  list = list.append(3);

  println!("{}", list.stringify());
}
