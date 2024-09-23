// Now try and implement a doubly linked version. Give an explanation
// for why this doesn't work.

// Commented out since it doesn't compile as noted above

/*  
struct Node {
    val: i32,
    next: Link,
    prev: Link,
}

type Link = Option<Box<Node>>;

pub struct LinkedStack {
    head: Link,
    tail: Link,
}

impl LinkedStack {
    fn new() -> Self {
        LinkedStack {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, val: i32) {
        match self.head.take() {
            None => {
                self.head = {
                    Some(Box::new(Node {
                        val,
                        next: None,
                        prev: None,
                    }))
                };
            }
            Some(mut prev_head ) => {
                let mut next_head = Some(Box::new(Node {
                    val,
                    next: None,
                    prev: Some(prev_head)
                }));
                // here we encounter an issue as we have a circular ownership problem
                // the compiler returns "value partially assigned here after move"
                prev_head.next = next_head.take();
                self.head = next_head.take();
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(mut node) => {
                // get value removed
                let val: Option<i32> = Some(node.val);
                // go back one
                let prev = node.prev.take();
                // set next of prev to None
                let mut prev = match prev {
                    None => None,
                    Some(mut prev) => {
                        prev.next = None;
                        Some(prev)
                    }
                };
                // move prev onto head
                self.head = prev.take();
                // return value
                val
            }
        }
    }
}

*/