use std::ptr::NonNull;

struct Node<T> {
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
    val: T,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            prev: None,
            next: None,
            val,
        }
    }
}

#[derive(Default)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    size: i32,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }
}

impl<T> LinkedList<T> {
    pub fn push(&mut self, val: T) {
        let mut node = Box::new(Node::new(val));

        node.prev = self.tail;

        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
        }

        self.tail = node_ptr;
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.tail.map(|tail_ptr| unsafe {
            self.tail = tail_ptr.as_ref().prev;
            if let Some(mut node) = self.tail {
                node.as_mut().next = None;
            }
            Box::from_raw(tail_ptr.as_ptr()).val
        })
    }

    pub fn head(&self) -> Option<&T> {
        self.head.map(|head_ptr| unsafe { &head_ptr.as_ref().val })
    }

    pub fn head_mut(&mut self) -> Option<&mut T> {
        self.head
            .map(|mut head_ptr| unsafe { &mut head_ptr.as_mut().val })
    }

    pub fn tail(&self) -> Option<&T> {
        self.tail.map(|tail_ptr| unsafe { &tail_ptr.as_ref().val })
    }

    pub fn tail_mut(&mut self) -> Option<&mut T> {
        self.tail
            .map(|mut tail_ptr| unsafe { &mut tail_ptr.as_mut().val })
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn linked_list_for_string() {
        let mut list = LinkedList::<String>::new();

        list.push("A".to_string());
        list.push("B".to_string());
        list.push("C".to_string());
        assert_eq!(3, list.size);

        assert_eq!("A", list.head().unwrap());
        assert_eq!("C", list.tail().unwrap());

        list.head_mut().unwrap().push('1');
        list.tail_mut().unwrap().push('3');

        assert_eq!("A1", list.head().unwrap());
        assert_eq!("C3", list.tail().unwrap());

        assert_eq!("C3", list.pop().unwrap());
        assert_eq!("B", list.pop().unwrap());
        assert_eq!("A1", list.pop().unwrap());
        assert_eq!(None, list.pop());
    }
}
