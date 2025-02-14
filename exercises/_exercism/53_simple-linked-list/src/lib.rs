struct Node<T> {
    element: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Self {
            element,
            next: None,
        }
    }

    fn next_as_ref(&self) -> Option<&Self> {
        match &self.next {
            Some(node) => Some(node),
            None => None,
        }
    }

    fn next_as_mut(&mut self) -> Option<&mut Self> {
        match &mut self.next {
            Some(node) => Some(node),
            None => None,
        }
    }

    fn push(&mut self, element: T) {
        if self.next.is_some() {
            panic!("Current node already have a next");
        };
        self.next = Some(Box::new(Self::new(element)));
    }

    fn is_second_last(&self) -> bool {
        match &self.next {
            Some(node) => node.next.is_none(),
            None => false,
        }
    }

    fn unwrap(self) -> T {
        if self.next.is_some() {
            panic!("Cannot unwrap nodes with some next")
        }
        self.element
    }

    fn peek(&self) -> &T {
        &self.element
    }

    fn take_next(&mut self) -> Option<T> {
        if self
            .next
            .as_ref()
            .map(|next_node| next_node.next.is_some())
            .unwrap_or(false)
        {
            panic!("Cannot take nodes with some next")
        }
        self.next.take().map(|node| node.element)
    }
}

pub struct SimpleLinkedList<T> {
    head: Option<Node<T>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let (mut current, mut length) = match &self.head {
            Some(node) => (node, 1),
            None => return 0,
        };
        while let Some(node) = current.next_as_ref() {
            current = node;
            length += 1
        }
        length
    }

    pub fn push(&mut self, element: T) {
        let mut current = match &mut self.head {
            Some(node) => node,
            None => {
                self.head = Some(Node::new(element));
                return;
            }
        };
        while current.next_as_ref().is_some() {
            current = current.next_as_mut().unwrap();
        }
        current.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        let mut current = match self.head.as_ref() {
            Some(node) if node.next_as_ref().is_none() => {
                return Some(self.head.take().unwrap().unwrap());
            }
            None => return None,
            _ => self.head.as_mut().unwrap(),
        };

        while !current.is_second_last() {
            current = current.next_as_mut().unwrap();
        }
        current.take_next()
    }

    pub fn peek(&self) -> Option<&T> {
        let mut current = match &self.head {
            Some(node) => node,
            None => return None,
        };
        while current.next_as_ref().is_some() {
            current = current.next_as_ref().unwrap();
        }
        Some(current.peek())
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        todo!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        todo!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        todo!()
    }
}
