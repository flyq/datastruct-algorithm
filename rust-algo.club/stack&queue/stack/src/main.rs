fn main() {
    println!("Hello, world!");
}

pub struct Stack<T> {
    maxsize: usize,
    items: Vec<T>,
}

pub fn with_capacity(maxsize: usize) -> Stack<T> {
    Stack {
        maxsize,
        items: Vec::with_capacity(maxsize),
    }
}

pub fn push(&mut self, item: T) -> bool {
    if self.items.len() == self.maxsize {
        return false;
    }
    self.items.push(item);
    return true;
}

pub fn pop(&mut self) -> Option<T> {
    self.items.pop()
}

pub fn size(&self) -> usize {
    self.items.len()
}

pub fn peek(&self) -> Option<&T> {
    self.items.last()
}
