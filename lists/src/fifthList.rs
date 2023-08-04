//-------------------------------------------------------

// use std::mem;

// pub struct List<T> {
//     head: Link<T>,
//     tail: Link<T>, // NEW!
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }

// impl<T> List<T> {
//     pub fn new() -> Self {
//         List { head: None, tail: None }
//     }

//     pub fn push(&mut self, elem: T) {
//         let new_tail = Box::new(Node {
//             elem: elem,
//             // 在尾端推入一个新节点时，新节点的下一个节点永远是 None
//             next: None,
//         });

//         // 让 tail 指向新的节点，并返回之前的 old tail
//         let old_tail = mem::replace(&mut self.tail, Some(new_tail));

//         match old_tail {
//             Some(mut old_tail) => {
//                 // 若 old tail 存在，则让该节点指向新的节点
//                 old_tail.next = Some(new_tail);
//             }
//             None => {
//                 // 否则，将 head 指向新的节点
//                 self.head = Some(new_tail);
//             }
//         }
//     }
// }

//-------------------------------------------------------

// pub struct List<T> {
//     head: Link<T>,
//     tail: Option<&mut Node<T>>, // NEW!
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }

// impl<T> List<T> {
//     pub fn new() -> Self {
//         List { head: None, tail: None }
//     }

//     pub fn push(&mut self, elem: T) {
//         let new_tail = Box::new(Node {
//             elem: elem,
//             next: None,
//         });

//         let new_tail = match self.tail.take() {
//             Some(old_tail) => {
//                 old_tail.next = Some(new_tail);
//                 old_tail.next.as_deref_mut()
//             }
//             None => {
//                 self.head = Some(new_tail);
//                 self.head.as_deref_mut()
//             }
//         };

//         self.tail = new_tail;
//     }
// }

//-------------------------------------------------------

// pub struct List<'a, T> {
//     head: Link<T>,
//     tail: Option<&'a mut Node<T>>, // NEW!
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }



// impl<'a, T> List<'a, T> {
//     pub fn new() -> Self {
//         List { head: None, tail: None }
//     }

//     pub fn push(&'a mut self, elem: T) {
//         let new_tail = Box::new(Node {
//             elem: elem,
//             next: None,
//         });

//         let new_tail = match self.tail.take() {
//             Some(old_tail) => {
//                 old_tail.next = Some(new_tail);
//                 old_tail.next.as_deref_mut()
//             }
//             None => {
//                 self.head = Some(new_tail);
//                 self.head.as_deref_mut()
//             }
//         };

//         self.tail = new_tail;
//     }

//     pub fn pop(&'a mut self) -> Option<T> {
//         self.head.take().map(|head| {
//             let head = *head;
//             self.head = head.next;
    
//             if self.head.is_none() {
//                 self.tail = None;
//             }
    
//             head.elem
//         })
//     }
// }

// mod test {
//     use super::List;
//     #[test]
//     fn basics() {
//         let mut list = List::new();

//         // Check empty list behaves right
//         assert_eq!(list.pop(), None);

//         // Populate list
//         list.push(1);
//         list.push(2);
//         list.push(3);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(1));
//         assert_eq!(list.pop(), Some(2));

//         // Push some more just to make sure nothing's corrupted
//         list.push(4);
//         list.push(5);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(3));
//         assert_eq!(list.pop(), Some(4));

//         // Check exhaustion
//         assert_eq!(list.pop(), Some(5));
//         assert_eq!(list.pop(), None);
//     }
// }

//-------------------------------------------------------

// use std::ptr;
// pub struct List<T> {
//     head: Link<T>,
//     tail: *mut Node<T>, // DANGER DANGER
// }

// type Link<T> = Option<Box<Node<T>>>;

// struct Node<T> {
//     elem: T,
//     next: Link<T>,
// }

// impl<T> List<T> {
//     pub fn new() -> Self {
//         List { head: None, tail: ptr::null_mut() }
//     }

//     pub fn push(&mut self, elem: T) {
//         let mut new_tail = Box::new(Node {
//             elem: elem,
//             next: None,
//         });
    
//         let raw_tail: *mut _ = &mut *new_tail;
    
//         // .is_null 会检查是否为 null, 在功能上等价于 `None` 的检查
//         if !self.tail.is_null() {
//             // 如果 old tail 存在，那将其指向新的 tail
//             unsafe {
//                 (*self.tail).next = Some(new_tail);
//             }
//         } else {
//             // 否则让 head 指向新的 tail
//             self.head = Some(new_tail);
//         }
    
//         self.tail = raw_tail;
//     }

//     pub fn pop(&mut self) -> Option<T> {
//         self.head
//         .take()
//         .map(|head| {
//             let head = *head;
//             self.head = head.next;
    
//             if self.head.is_none() {
//                 self.tail = ptr::null_mut();
//             }
    
//             head.elem
//         })
//     }
    
// }

// #[cfg(test)]
// mod test {
//     use super::List;
//     #[test]
//     fn basics() {
//         let mut list = List::new();

//         // Check empty list behaves right
//         assert_eq!(list.pop(), None);

//         // Populate list
//         list.push(1);
//         list.push(2);
//         list.push(3);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(1));
//         assert_eq!(list.pop(), Some(2));

//         // Push some more just to make sure nothing's corrupted
//         list.push(4);
//         list.push(5);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(3));
//         assert_eq!(list.pop(), Some(4));

//         // Check exhaustion
//         assert_eq!(list.pop(), Some(5));
//         assert_eq!(list.pop(), None);
//         assert_eq!(list.pop(), None); 

//         // Check the exhaustion case fixed the pointer right
//         list.push(6);
//         list.push(7);

//         // Check normal removal
//         assert_eq!(list.pop(), Some(6));
//         assert_eq!(list.pop(), Some(7));
//         assert_eq!(list.pop(), None);
//     }
// }

//-------------------------------------------------------
use std::ptr;
type Link<T> = *mut Node<T>; // 嘀，新的好人卡，请查收

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        unsafe {
            // 一开始就将 Box 转换成裸指针
            let new_tail = Box::into_raw(Box::new(Node {
                elem: elem,
                next: ptr::null_mut(),
            }));
    
            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }
    
            self.tail = new_tail;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;
    
                if self.head.is_null() {
                    self.tail = ptr::null_mut();
                }
    
                Some(head.elem)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unsafe {
            self.head.as_ref().map(|node| &node.elem)
        }
    }
    
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().map(|node| &mut node.elem)
        }
    }
    
    
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() { }
    }
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter { next: self.head
                .as_ref() }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        unsafe {
            IterMut { next: self.head
                .as_mut() }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next
                .map(|node| {
                self.next = node.next
                            .as_ref();
                &node.elem
            })
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node| {
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}


#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
