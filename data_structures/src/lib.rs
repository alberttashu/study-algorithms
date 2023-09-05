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

        pub fn count(&self) -> usize {
            let mut count = 0;
            let mut current = &self.head;
            while !current.is_none() {
                count = count + 1;
                current = &current.as_ref().unwrap().next;
            }
            count
        }

        // it created borrow when executed, so better to use a version with cloning here
        // pub fn to_array(&self) -> Vec<&T> {
        //     let mut result = Vec::new();
        //     let mut current = &self.head;
        //
        //     while let Some(node) = current {
        //         result.push(&node.value);
        //         current = &node.next;
        //     }
        //
        //     result
        // }

        pub fn to_array(&self) -> Vec<T>
        where
            T: Clone, // Ensure T implements Clone
        {
            let mut result = Vec::new();
            let mut current = &self.head;

            while let Some(node) = current {
                result.push(node.value.clone()); // Clone each element
                current = &node.next;
            }

            result
        }

        pub fn reverse(&mut self) {
            let mut prev = None;
            let mut current = self.head.take();

            while let Some(mut node) = current.take() {
                let next = node.next.take();
                node.next = prev.take();
                prev = Some(node);
                current = next;
            }

            self.head = prev;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_up_list_with_items_should_return_proper_count() {
        let mut list: singly_linked_list::LinkedList<i32> = singly_linked_list::LinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);

        let count = list.count();

        assert_eq!(count, 5);
    }

    #[test]
    fn reserve_should_reverse_list() {
        let mut list = singly_linked_list::LinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        let initial = list.to_array();

        list.reverse();

        let reversed = list.to_array();

        assert_eq!(reversed, initial.iter().rev().copied().collect::<Vec<_>>())
    }
}
