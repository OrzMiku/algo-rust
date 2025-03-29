#[derive(Debug)]
struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T, next: Option<Box<Node<T>>>) -> Node<T> {
        Node { value, next }
    }
}

/// A singly-linked list implementation.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use algo_rust::data_structs::LinkedList;
///
/// let mut list = LinkedList::new();
///
/// // Push elements
/// list.push(1);
/// list.push(2);
/// list.push(3);
///
/// // Check length
/// assert_eq!(list.length(), 3);
///
/// // Pop elements
/// assert_eq!(list.pop(), Some(3));
/// assert_eq!(list.pop(), Some(2));
///
/// // Peek at the front
/// assert_eq!(list.peek(), Some(&1));
///
/// // Check if empty
/// assert!(!list.is_empty());
/// ```
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    /// Creates an empty `LinkedList`.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let list: LinkedList<i32> = LinkedList::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    /// Adds an element to the front of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.peek(), Some(&2));
    /// ```
    pub fn push(&mut self, value: T) {
        self.length += 1;
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
    }

    /// Removes and returns the element at the front of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.pop(), Some(2));
    /// assert_eq!(list.pop(), Some(1));
    /// assert_eq!(list.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.length -= 1;
            self.head = node.next;
            node.value
        })
    }

    /// Returns a reference to the element at the front of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.peek(), None);
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.peek(), Some(&2));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// Returns a mutable reference to the element at the front of the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// if let Some(value) = list.peek_mut() {
    ///     *value = 3;
    /// }
    /// assert_eq!(list.pop(), Some(3));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    /// Returns the number of elements in the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert_eq!(list.length(), 0);
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.length(), 2);
    /// ```
    pub fn length(&self) -> usize {
        self.length
    }

    /// Returns `true` if the list contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// assert!(list.is_empty());
    /// list.push(1);
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Clears the list, removing all elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    /// assert_eq!(list.length(), 2);
    /// list.clear();
    /// assert!(list.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Creates an iterator that consumes the list.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    ///
    /// let mut iter = list.into_iter();
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), Some(1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }

    /// Creates an iterator over the list's elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    ///
    /// let mut iter = list.iter();
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// Creates a mutable iterator over the list's elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use algo_rust::data_structs::LinkedList;
    /// let mut list = LinkedList::new();
    /// list.push(1);
    /// list.push(2);
    ///
    /// for elem in list.iter_mut() {
    ///     *elem *= 2;
    /// }
    ///
    /// assert_eq!(list.pop(), Some(4));
    /// assert_eq!(list.pop(), Some(2));
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
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

/// An iterator that consumes the list.
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

/// An iterator over the list's elements.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

/// A mutable iterator over the list's elements.
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.head.take().map(|node| {
            self.list.head = node.next;
            node.value
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.value
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.value
        })
    }
}
