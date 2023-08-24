#[allow(dead_code)]
mod singly_linked_list {
    pub struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
    }

    pub struct LinkedList<T> {
        head: Option<Box<Node<T>>>,
    }

    impl<T> LinkedList<T> {
        pub fn new() -> Self {
            LinkedList { head: None }
        }

        pub fn push(&mut self, value: T) {
            let new_node = Box::new(Node {
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
        }

        pub fn pop(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                self.head = node.next;
                node.value
            })
        }

        pub fn is_empty(&self) -> bool {
            self.head.is_none()
        }

        pub fn peek(&mut self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.value)
        }

        pub fn count(&self) -> i32 {
            let mut count = 0;
            let mut current = &self.head;
            while !current.is_none() {
                count = count + 1;
                current = &current.as_ref().unwrap().next;
            }
            count
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list: singly_linked_list::LinkedList<i32> = singly_linked_list::LinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);

        let count = list.count();

        assert_eq!(count, 5);
    }
}
