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
        self.next.as_deref()
    }

    fn next_as_mut(&mut self) -> Option<&mut Self> {
        self.next.as_deref_mut()
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
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut stack: Vec<T> = Vec::new();
        while let Some(element) = self.pop() {
            stack.push(element)
        }
        stack.into_iter().collect()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = Self::new();
        for element in iter {
            list.push(element)
        }
        list
    }
}

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut stack: Vec<T> = Vec::new();
        while let Some(element) = linked_list.pop() {
            stack.push(element)
        }
        stack.into_iter().rev().collect()
    }
}
