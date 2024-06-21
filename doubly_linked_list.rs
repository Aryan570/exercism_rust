// unsafe unsafe unsafe
// Todo : Do this exercise With NonNull
use core::panic;

// this module adds some functionality based on the required implementations
// here like: `LinkedList::pop_back` or `Clone for LinkedList<T>`
// You are free to use anything in it, but it's mainly for the test framework.
mod pre_implemented;
pub struct Node<T>{
    prev : *mut Node<T>,
    next : *mut Node<T>,
    value : T
}
#[derive(Clone)]
pub struct LinkedList<T>{
    tail : *mut Node<T>,
    head : *mut Node<T>,
    len : usize
}
impl<T> Node<T>{
    fn new(value : T)-> Self{
        Node { prev: std::ptr::null_mut(), next: std::ptr::null_mut(), value }
    }
}
pub struct Cursor<'a, T>{
    list : &'a mut LinkedList<T>,
    current : *mut Node<T>
}

pub struct Iter<'a, T>(Option<&'a Node<T>>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { tail : std::ptr::null_mut(), head : std::ptr::null_mut() , len : 0 }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for LinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Return a cursor positioned on the front element
    pub fn cursor_front(&mut self) -> Cursor<'_, T> {
        let head = self.head;
        Cursor { list: self, current:  head}
    }

    /// Return a cursor positioned on the back element
    pub fn cursor_back(&mut self) -> Cursor<'_, T> {
        let tail = self.tail;
        Cursor { list: self, current: tail }
    }

    /// Return an iterator that moves from front to back
    pub fn iter(&self) -> Iter<'_, T> {
        if self.head.is_null(){return Iter(None);}
        unsafe {Iter(Some(&(*self.head)))}
    }
}
impl<T> Drop for LinkedList<T>{
    fn drop(&mut self) {
        let mut ptr = self.cursor_front();
        while ptr.take().is_some(){}
    }
}

// the cursor is expected to act as if it is at the position of an element
// and it also has to work with and be able to insert into an empty list.
impl<T> Cursor<'_, T> {
    /// Take a mutable reference to the current element
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if self.current.is_null(){return None;}
        unsafe{
            let ele = &mut (*self.current).value;
            Some(ele)
        }
    }

    /// Move one position forward (towards the back) and
    /// return a reference to the new position
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<&mut T> {
        if self.current.is_null(){panic!()}
        unsafe { self.current = (*self.current).next;
            self.peek_mut()
        }
    }

    /// Move one position backward (towards the front) and
    /// return a reference to the new position
    pub fn prev(&mut self) -> Option<&mut T> {
        if self.current.is_null(){panic!()}
        unsafe {
            self.current = (*self.current).prev;
        }
        self.peek_mut()
    }

    /// Remove and return the element at the current position and move the cursor
    /// to the neighboring element that's closest to the back. This can be
    /// either the next or previous position.
    pub fn take(&mut self) -> Option<T> {
        if self.current.is_null() || self.list.len == 0 {return None;}
        let val = self.current;
        if self.list.len == 1{
            self.current = std::ptr::null_mut();
            self.list.head = std::ptr::null_mut();
            self.list.tail = std::ptr::null_mut();
            self.list.len = 0;
        }else if self.current == self.list.head{
            unsafe{self.current = (*self.current).next;}
            self.list.head = self.current;
            self.list.len -= 1;
        }else if self.current == self.list.tail{
            unsafe {self.current = (*self.current).prev;}
            self.list.tail = self.current;
            self.list.len -= 1;
        }else {
            unsafe {
                let nxt = (*self.current).next;
                let pre = (*self.current).prev;
                (*nxt).prev = pre;
                (*pre).next = nxt;
                self.current = nxt; 
            }
        }
        unsafe{
            let c = Box::from_raw(val);
            Some(c.value)
        }
    }

    pub fn insert_after(&mut self, element: T) {
        let new_node = Box::into_raw(Box::new(Node::new(element)));
        if self.current.is_null() || self.list.len == 0{
            self.list.head = new_node;
            self.list.tail = self.list.head;
            self.current = self.list.head;
        }else if self.list.len == 1{
            unsafe{
                (*self.list.head).next = new_node;
                (*new_node).prev = self.list.head;
                self.list.tail = new_node;
            }
        }else{
            if self.current == self.list.tail{
                self.list.tail = new_node;
            }
            unsafe{
                let nxt = (*self.current).next;
                (*new_node).prev = self.current;
                (*new_node).next = nxt;
                (*self.current).next = new_node;
                if !nxt.is_null(){
                    (*nxt).prev = new_node; 
                }
            }
        }
        self.list.len += 1;
    }

    pub fn insert_before(&mut self, element: T) {
        let new_node = Box::into_raw(Box::new(Node::new(element)));
        if self.current.is_null() || self.list.len == 0{
            self.list.head = new_node;
            self.list.tail = self.list.head;
            self.current = self.list.head;
        }else if self.list.len == 1{
            unsafe{
                (*self.list.head).prev = new_node;
                (*new_node).next = self.list.head;
            }
            self.list.head = new_node;
        }else{
            if self.current == self.list.head{
                self.list.head = new_node;
            }
            unsafe{
                let pre = (*self.current).prev;
                (*new_node).prev = pre;
                (*new_node).next = self.current;
                (*self.current).prev = new_node;
                if !pre.is_null(){
                    (*pre).next = new_node; 
                }
            }
        }
        self.list.len += 1;
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        let val = &self.0?.value;
        if self.0?.next.is_null(){
            self.0 = None;
        }else{
            unsafe {
                self.0 = Some(&*self.0?.next);
            }
        }
        Some(val)
    }
}
