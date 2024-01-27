use std::cmp::Ordering;
use std::ops::{Index, IndexMut};

pub struct LinkedNode<T> {
    data: T,
    next: Option<Box<LinkedNode<T>>>,
}

impl<T> LinkedNode<T> {
    pub fn new(v: T) -> Self {
        LinkedNode {
            data: v,
            next: None,
        }
    }

    #[inline]
    pub fn push(&mut self, v: T) {
        match &mut self.next {
            None => {
                let val = v;
                let node = Box::new(LinkedNode::new(val));
                self.next = Some(node)
            }
            Some(ntx) => {
                ntx.push(v)
            }
        }
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        match self.next.as_mut() {
            None => {
                None
            }
            Some(ntx) => {
                match ntx.next.as_mut() {
                    None => {
                        self.next.take().map(|x| {
                            (*x).data
                        })
                    }
                    Some(_) => {
                        ntx.as_mut().pop()
                    }
                }
            }
        }
    }

    pub fn iter(&self) -> LinkedIter<T> {
        LinkedIter {
            p: Some(self)
        }
    }

    pub fn iter_mut(&mut self) -> LinkedIterMut<T> {
        LinkedIterMut {
            p: Some(self)
        }
    }
}

impl<T> PartialOrd for LinkedNode<T>
    where T: PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.data < other.data {
            Some(Ordering::Less)
        } else if self.data > other.data {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }

    fn lt(&self, other: &Self) -> bool {
        self.data < other.data
    }

    fn le(&self, other: &Self) -> bool {
        self.data <= other.data
    }

    fn gt(&self, other: &Self) -> bool {
        self.data > other.data
    }

    fn ge(&self, other: &Self) -> bool {
        self.data >= other.data
    }
}

impl<T> PartialEq for LinkedNode<T>
    where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<'a, T> Index<usize> for LinkedNode<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        let mut count = 0;
        let mut p = &self.data;
        for x in self.iter() {
            if count == index {
                p = x;
                break;
            }
            count += 1;
        }
        p
    }
}

impl<'a, T> IndexMut<usize> for LinkedNode<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let mut p = self;
        for _ in 0..index {
            match p.next.as_mut() {
                None => {
                    panic!("out of memory!!!")
                }
                Some(ntx) => {
                    p = &mut *ntx;
                }
            }
        }
        &mut p.data
    }
}

pub struct LinkedIter<'a, T> {
    p: Option<&'a LinkedNode<T>>,
}

impl<'a, T> Iterator for LinkedIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.p {
            None => { None }
            Some(cur) => {
                self.p = cur.next.as_ref().map(|ntx| &**ntx);
                Some(&cur.data)
            }
        }
    }
}


pub struct LinkedIterMut<'a, T> {
    p: Option<&'a mut LinkedNode<T>>,
}

impl<'a, T> Iterator for LinkedIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.p.take().map(|node| {
            self.p = node.next.as_deref_mut();
            &mut node.data
        })
    }
}


#[test]
fn test_linked_list() {
    let mut node: LinkedNode<i32> = LinkedNode::new(0);
    node.push(1);
    node.push(2);
    node.push(3);
    node.push(4);
    node.push(5);
    node.push(6);


    for x in node.iter() {
        print!("{:?} ", x);
    }

    println!();
    for x in node.iter_mut() {
        *x += 10;
    }

    for x in node.iter() {
        print!("{:?} ", x);
    }

    println!();

    let v = node.pop();
    println!("{:?}", v);
    assert_eq!(v, Some(16));
    let v = node.pop();
    println!("{:?}", v);
    assert_eq!(v, Some(15));
    let v = node.pop();
    println!("{:?}", v);
    assert_eq!(v, Some(14));
    let v = node.pop();
    assert_eq!(v, Some(13));

    for x in node.iter() {
        print!("{:?} ", x);
    }
    println!();

    let v1 = node[0];
    println!("{:?}", v1);
    assert_eq!(v1, 10);
    let v1 = node[1];
    println!("{:?}", v1);
    assert_eq!(v1, 11);

    node[0] += 1000;
    let v1 = node[0];
    println!("{:?}", v1);
    assert_eq!(v1, 1010);
    node[1] += 1000;
    let v1 = node[1];
    println!("{:?}", v1);
    assert_eq!(v1, 1011);

    node[2] += 1000;
    let v1 = node[2];
    println!("{:?}", v1);
    assert_eq!(v1, 1012);

    assert_ne!(node[0], node[1]);
    node[1] = 1010;
    assert_eq!(node[0], node[1]);

    assert!(node[2] > node[0]);
    assert!(node[1] < node[2]);
    assert!(node[0] <= node[1]);
    assert!(node[0] >= node[1]);


    let mut x = vec![1, 2, 3];
    for a in x.iter_mut() {
        *a = 100;
    }
}