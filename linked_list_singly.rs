pub struct SimpleLinkedList<T> {
    head : Option<Box<Node<T>>>,
    len : usize
}
struct Node<T>{
    value : T,
    next : Option<Box<Node<T>>>
}

impl<T> SimpleLinkedList<T> {
    // its the head with NONE
    pub fn new() -> Self {
        SimpleLinkedList { head: None , len : 0}
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }
    pub fn len(&self) -> usize {
        self.len
    }
    // fuck !! push in linked list means creating new head (new node as head)
    pub fn push(&mut self, element: T) {
        let node = Box::new(Node {value : element , next : self.head.take()});
        self.head = Some(node);
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {return None;}
        self.len -= 1;
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        if self.len == 0 {return Self::new();}
        let mut reversed = Self::new();
        let mut curr = self.head;
        while let Some(node) = curr {
            reversed.push(node.value);
            curr = node.next;
        }
        reversed
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut ll = Self::new();
        for val in _iter{
            ll.push(val);
        }
        ll
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut v: Vec<T> = Vec::new();
        if _linked_list.len == 0 {return v;}
        _linked_list = _linked_list.rev();
        let mut curr = _linked_list.head;
        while let Some(node) = curr{
            v.push(node.value);
            curr = node.next;
        }
        v
    }
}
