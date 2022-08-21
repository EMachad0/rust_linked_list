use std::sync::Arc;

// Arc make it thread safe if T: Send + Sync
type Link<T> = Option<Arc<Node<T>>>;

#[derive(Debug)]
pub struct List<T>(Link<T>);

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List(None)
    }

    pub fn push(&self, value: T) -> Self {
        let new_node = Node {
            value,
            next: self.0.clone(),
        };
        List(Some(Arc::new(new_node)))
    }

    pub fn tail(&self) -> Self {
        let node = self.0.clone();
        List(node.and_then(|node| node.next.clone()))
    }

    pub fn head(&self) -> Option<&T> {
        self.0.as_ref().map(|node| &node.value)
    }
}

// Custom code within the destructor.
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.0.take();
        while let Some(node) = head {
            if let Ok(mut node) = Arc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
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

impl<T> List<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        let node = self.0.as_deref();
        Iter(node)
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn creates_an_empty_list() {
        let list: List<i32> = List::new();
        assert_eq!(None, list.head())
    }

    #[test]
    fn pushes_element() {
        let list = List::new();
        let value = 5;
        let list = list.push(value);
        assert_eq!(Some(&value), list.head());
    }

    #[test]
    fn tail() {
        let list = List::new();
        assert_eq!(list.head(), None);

        let list = list.push(1).push(2).push(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));

        let list = list.tail();
        assert_eq!(list.head(), None);

        // Make sure empty tail works
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter() {
        let list = List::new().push(1).push(2).push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
