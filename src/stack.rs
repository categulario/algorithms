struct Node<T> {
    value: T,
    next: Box<Option<Node<T>>>,
}

pub struct Stack<T> {
    head: Box<Option<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            head: Box::new(None),
        }
    }

    pub fn push(&mut self, value: T) {
        match self.head.take() {
            Some(node) => {
                let new_head = Node {
                    value,
                    next: Box::new(Some(node)),
                };

                self.head.replace(new_head);
            },
            None => {
                let new_head = Node {
                    value,
                    next: Box::new(None),
                };

                self.head.replace(new_head);
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;

            node.value
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();

        stack.push(8);
        stack.push(7);
        stack.push(6);

        assert_eq!(stack.pop(), Some(6));

        stack.push(5);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(7));
        assert_eq!(stack.pop(), Some(8));
        assert_eq!(stack.pop(), None);

        stack.push(1);

        assert_eq!(stack.pop(), Some(1));
    }
}
