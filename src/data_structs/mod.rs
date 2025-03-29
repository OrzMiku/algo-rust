#[derive(Debug)]
struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node {value, next}
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    /// Constructs a new empty linked list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::<i32>::new();
    /// assert_eq!(list.pop(), None);
    /// ```
    /// 
    pub fn new() -> LinkedList<T> {
        LinkedList::<T> { head: None, length: 0 }
    }

    /// Pushes a new value onto the front of the list.
    ///
    /// # Examples
    /// 
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::<i32>::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    /// assert_eq!(list.pop(), Some(3));
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.pop(), Some(1));
    /// assert_eq!(list.pop(), None);
    /// ```
    /// 
    pub fn push(&mut self, value: T) {
        self.length += 1;
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
    }

    /// Pops a value from the front of the list.
    ///
    /// # Examples
    /// 
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::<i32>::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    /// assert_eq!(list.pop(), Some(3));
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.pop(), Some(1));
    /// assert_eq!(list.pop(), None);
    /// ```
    /// 
    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                self.length -= 1;
                self.head = node.next;
                Some(node.value)
            }
            None => None,
        }
    }

    /// Peeks at the value at the front of the list without removing it.
    ///
    /// # Examples
    /// 
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::<i32>::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    /// assert_eq!(list.peek(), Some(&3));
    /// assert_eq!(list.pop(), Some(3));
    /// assert_eq!(list.peek(), Some(&2));
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.peek(), Some(&1));
    /// assert_eq!(list.pop(), Some(1));
    /// assert_eq!(list.peek(), None);
    /// ```
    ///
    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    /// Peeks at the value at the front of the list mutably without removing it.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::<i32>::new();
    /// list.push(1);
    /// if let Some(value) = list.peek_mut() {
    ///    *value = 2;
    ///    assert_eq!(list.pop(), Some(2));
    /// }
    /// assert_eq!(list.peek(), None); 
    /// ```
    ///
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    /// Returns the length of the list.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::<i32>::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    /// assert_eq!(list.length(), 3);
    /// list.pop();
    /// list.pop();
    /// list.pop();
    /// list.pop();
    /// assert_eq!(list.length(), 0);
    /// ```
    /// 
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns true if the list is empty, false otherwise.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::<i32>::new();
    /// assert_eq!(list.is_empty(), true);
    /// list.push(1);
    /// assert_eq!(list.is_empty(), false);
    /// list.pop();
    /// assert_eq!(list.is_empty(), true);
    /// ```
    /// 
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Clears the list, removing all elements.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::<i32>::new();
    /// list.push(1);
    /// list.push(2);
    /// list.push(3);
    /// assert_eq!(list.length(), 3);
    /// list.clear();
    /// assert_eq!(list.length(), 0);
    /// assert_eq!(list.pop(), None);
    /// ```
    ///
    pub fn clear(&mut self) {
        self.head = None;
        self.length = 0;
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node) = current {
            current = node.next;
        }
    }
}

impl<T: Clone> Clone for LinkedList<T> {
    fn clone(&self) -> Self {
        let mut new_list = Self::new();
        let mut current = &self.head;
        while let Some(node) = current {
            new_list.push(node.value.clone());
            current = &node.next;
        }
        new_list
    }
}