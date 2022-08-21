use std::ptr::null_mut;

type Link<T> = *mut Node<T>;

#[derive(Debug)]
pub struct Queue<T> {
    head: Link<T>,
    last: *mut Node<T>,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            next: null_mut(),
        }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: null_mut(),
            last: null_mut(),
        }
    }

    pub fn push(&mut self, value: T) {
        let new_last = Box::new(Node::new(value));
        let new_last_prt: *mut _ = Box::into_raw(new_last);
        if self.last.is_null() {
            self.head = new_last_prt;
        } else {
            unsafe {
                (*self.last).next = new_last_prt;
            }
        };

        self.last = new_last_prt;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            unsafe {
                let node = Box::from_raw(self.head);
                self.head = node.next;
                if self.head.is_null() {
                    self.last = null_mut();
                }
                Some(node.value)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.value) }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe { self.head.as_mut().map(|node| &mut node.value) }
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

// IntoIter for consumed iteration
pub struct IntoIter<T>(Queue<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T> Queue<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

// Iter for referenced iteration
pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            unsafe {
                self.0 = node.next.as_ref();
            }
            &node.value
        })
    }
}

impl<'a, T> IntoIterator for &'a Queue<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Queue<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        unsafe { Iter(self.head.as_ref()) }
    }
}

// IterMut for mutable referenced iteration
pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            unsafe {
                self.0 = node.next.as_mut();
            }
            &mut node.value
        })
    }
}

impl<'a, T> IntoIterator for &'a mut Queue<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Queue<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe { IterMut(self.head.as_mut()) }
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn basics() {
        let mut list = Queue::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = Queue::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.peek_mut(), Some(&mut 1));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = Queue::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Queue::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = Queue::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
