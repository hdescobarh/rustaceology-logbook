struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

//  Mutates at the beginning (O(1)). Last In, First Out.
pub struct SimpleLinkedList<T> {
    head: Option<Node<T>>,
    length: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn push(&mut self, element: T) {
        self.head = Some(Node {
            element,
            next: self.head.take().map(Box::new),
        });
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            self.head = old_head.next.map(|boxed_node| *boxed_node);
            self.length -= 1;
            old_head.element
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.element)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        self.into_iter().collect()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for element in iter {
            list.push(element);
        }
        list
    }
}

pub struct SimpleLinkedListIter<T>(SimpleLinkedList<T>);

impl<T> Iterator for SimpleLinkedListIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = SimpleLinkedListIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        SimpleLinkedListIter(self)
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(linked_list: SimpleLinkedList<T>) -> Self {
        let mut output: Vec<T> = linked_list.into_iter().collect();
        output.reverse();
        output
    }
}
