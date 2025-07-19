struct Node {
    data: i32,
    next_node: Option<Box<Node>>,
}
struct List {
    head: Option<Box<Node>>,
}
impl List {
    pub fn new() -> Self {
        return List { head: None };
    }
    pub fn is_empty(&self) -> bool {
        return self.head.is_some();
    }

    pub fn add_element(&mut self, data: i32) {
        let new_node = Box::new(Node {
            data,
            next_node: None,
        });

        match self.head.as_mut() {
            None => {
                self.head = Some(new_node);
            }
            Some(mut current) => {
                while let Some(ref mut next) = current.next_node {
                    current = next;
                }
                current.next_node = Some(new_node);
            }
        }
    }
}

fn main() {
    let lista = List::new();
    assert!(lista.is_empty());
}
