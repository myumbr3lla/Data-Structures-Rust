struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    value: i16,
}

impl Node {
    fn new(value: i16) -> Self {
        Node {
            left: None,
            right: None,
            value,
        }
    }

    fn insert(&mut self, value: i16) {
        if value < self.value {
            if let Some(left) = self.left.as_mut() {
                left.insert(value);
            } else {
                self.left = Some(Box::new(Node::new(value)));
            }
        } else {
            if let Some(right) = self.right.as_mut() {
                right.insert(value);
            } else {
                self.right = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn search(&self, value: i16) -> Option<&Node> {
        if value == self.value {
            return Some(self);
        }

        if value < self.value {
            self.left.as_ref()?.search(value)
        } else {
            self.right.as_ref()?.search(value)
        }
    }

    fn delete(&mut self, value: i16) -> Option<Box<Node>> {
        if value < self.value {
            if let Some(left) = self.left.as_mut() {
                left.delete(value);
            }
        } else if value > self.value {
            if let Some(right) = self.right.as_mut() {
                right.delete(value);
            }
        } else {
            if self.left.is_none() {
                return self.right.take();
            } else if self.right.is_none() {
                return self.left.take();
            }

            let min_right_value = self.right.as_mut().unwrap().min_node().value;
            self.value = min_right_value;
            self.right = self.right.as_mut().unwrap().delete(min_right_value);
        }

        Some(Box::new(self.clone()))
    }

    fn min_node(&self) -> &Node {
        if let Some(left) = &self.left {
            left.min_node()
        } else {
            self
        }
    }
}

fn new(slice: Vec<i16>) -> Option<Box<Node>> {
    let mut root = None;

    for &val in &slice {
        if let Some(node) = root.as_mut() {
            node.insert(val);
        } else {
            root = Some(Box::new(Node::new(val)));
        }
    }

    root
}

fn main() {
    let mut root = new(vec![5, 3, 8, 1, 4]).unwrap();

    root.insert(6);
    root.insert(7);

    if let Some(found_node) = root.search(4) {
        println!("Found node with value: {}", found_node.value);
    } else {
        println!("Node not found");
    }

    root.delete(3);
    if root.search(3).is_none() {
        println!("Node with value 3 deleted successfully");
    }
}
