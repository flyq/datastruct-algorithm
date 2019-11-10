fn main() {
    println!("Hello, world!");
}
struct Node<T> {
    elem: T,
    next: Box<Node<T>>,
}


pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
}


impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn clear(&mut self) {
        *self = Self::new();
    }
}

pub fn push_front(&mut self, elem: T) {
    let next = self.head.take(); // 1
    self.head = Some(Box::new(Node { elem, next })); // 2
}

pub fn pop_front(&mut self) -> Option<T> {
    let head = self.head.take(); // 1
    match head {
        Some(node) => {
            self.head = node.next;  // 2
            Some(node.elem)         // 3
        }
        None => None,
    }
}


pub fn insert_after(&mut self, pos: usize, elem: T) -> Result<(), usize> {
    let mut curr = &mut self.head;
    let mut pos_ = pos;

    while pos_ > 0 {                        // 1
        curr = match curr.as_mut() {
            Some(node) => &mut node.next,
            None => return Err(pos - pos_),
        };
        pos_ -= 1;
    }

    match curr.take() {                     // 2
        Some(mut node) => {   // Node A
            let new_node = Box::new(Node {  // 3: Node B
                elem,
                next: node.next,
            });
            node.next = Some(new_node);     // 4
            *curr = Some(node);             // 5
        }
        None => return Err(pos - pos_)
    }
    Ok(())
}

pub fn remove(&mut self, pos: usize) -> Option<T> {
    let mut curr = &mut self.head;
    let mut pos = pos;

    while pos > 0 {                // 1
        curr = match curr.as_mut() {
            Some(node) => &mut node.next,
            None => return None,
        };
        pos -= 1;
    }

    match curr.take() {            // 2
        Some(node) => { // Node A
            *curr = node.next;     // 3: node.next is Node B
            Some(node.elem)        // 4
        }
        None => None
    }
}

pub fn reverse(&mut self) {
    let mut prev = None;              // 1: prev -> Node P
    let mut curr = self.head.take();  // 2
    while let Some(mut node) = curr { // 3: node -> Node A
        let next = node.next;         // 3-1: next -> Node B
        node.next = prev.take();      // 3-2
        prev = Some(node);            // 3-3
        curr = next;                  // 3-4
    }
    self.head = prev.take(); // 4
}


impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();  // 1
        while let Some(mut node) = link { // 2
            link = node.next.take();      // 3
        }                                 // 4
    }
}

impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}


pub struct IntoIter<T>(SinglyLinkedList<T>);

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T: 'a> {
    next: Option<&'a mut Node<T>>,
}

pub struct IntoIter<T>(SinglyLinkedList<T>);      // 1

impl<T> Iterator for IntoIter<T> {                // 2
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> IntoIterator for SinglyLinkedList<T> {    // 3
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}


pub struct Iter<'a, T: 'a> {                    // 1
    next: Option<&'a Node<T>>,
}

impl<T> Iterator for Iter<'a, T> {
    type Item = &'a T;                          // 2
    fn next(&mut self) -> Option<Self::Item> {
        match self.next {
            Some(node) => {
                self.next = node.next.as_ref().map(|node| &**node); // 3, 4
                Some(&node.elem)
            }
            None => None,
        }
    }
}

impl<T> SinglyLinkedList<T> {
    // ...

    pub fn iter(&self) -> Iter<T> {             // 5
        Iter { next: self.head.as_ref().map(|node| &**node) } // 6
    }
}


impl<T: PartialEq> PartialEq for SinglyLinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        self.iter()
            .zip(other.iter())
            .all(|pair| pair.0 == pair.1)
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for elem in self.iter() {
            write!(f, "{:?} -> ", elem)?
        }
        Ok(())
    }
}
