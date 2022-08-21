use std::mem;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Stack<T>(Link<T>);

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack(None)
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: mem::take(&mut self.0),
        };

        self.0 = Some(Box::new(new_node))
    }

    pub fn pop(&mut self) -> Option<T> {
        let link = mem::take(&mut self.0);
        link.map(|node| {
            self.0 = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        let link = self.0.as_ref();
        link.map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        let link = self.0.as_mut();
        link.map(|node| &mut node.value)
    }
}

// Custom code within the destructor.
impl<T> Drop for Stack<T> {
    fn drop(&mut self) {
        let mut link = mem::take(&mut self.0);
        while let Some(mut node) = link {
            link = mem::take(&mut node.next);
        }
    }
}

// IntoIter for consumed iteration
pub struct IntoIter<T>(Stack<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> Stack<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

// Iter for referenced iteration
pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref();
            &node.value
        })
    }
}

impl<'a, T> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<T> Stack<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        let node = self.0.as_deref();
        Iter(node)
    }
}

// IterMut for mutable referenced iteration
pub struct IterMut<'a, T>(Option<&'a mut Node<T>>);

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.take().map(|node| {
            self.0 = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

impl<'a, T> IntoIterator for &'a mut Stack<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Stack<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let node = self.0.as_deref_mut();
        IterMut(node)
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn creates_an_empty_list() {
        let list: Stack<i32> = Stack::new();
        assert_eq!(None, list.peek())
    }

    #[test]
    fn pushes_element() {
        let mut list = Stack::new();
        let value = 5;
        list.push(value);
        assert_eq!(Some(&value), list.peek());
    }

    #[test]
    fn pops_empty() {
        let mut list: Stack<i32> = Stack::new();
        assert_eq!(None, list.pop())
    }

    #[test]
    fn pops() {
        let mut list = Stack::new();
        let value = 5;
        list.push(value);
        assert_eq!(Some(value), list.pop());
    }

    #[test]
    fn peek() {
        let mut list = Stack::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = Stack::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
