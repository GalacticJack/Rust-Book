#![allow(unused_variables)]
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Use indirection to prevent infinite recursion
    Nil,
}

use std::rc::Weak;
use std::{ops::Deref, rc::Rc};

use crate::List::{Cons, Nil};
fn box_example() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive list: {:?}", list);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn deref_trait() {
    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let zx = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *zx);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn drop_trait() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of func.");
}

enum RCList {
    RCons(i32, Rc<RCList>),
    RNil,
}
use crate::RCList::{RCons, RNil};
fn referance_counter() {
    let a = Rc::new(RCons(5, Rc::new(RCons(10, Rc::new(RNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RCons(4, Rc::clone(&a));
        println!("count after creatinc c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out fo scope = {}", Rc::strong_count(&a));
}

use crate::RefCycleList::{RCLCons, RCLNil};
use std::cell::RefCell;

#[derive(Debug)]
enum RefCycleList {
    RCLCons(i32, RefCell<Rc<RefCycleList>>),
    RCLNil,
}

impl RefCycleList {
    fn tail(&self) -> Option<&RefCell<Rc<RefCycleList>>> {
        match self {
            RCLCons(_, item) => Some(item),
            RCLNil => None,
        }
    }
}

fn ref_cycle_overflow() {
    let a = Rc::new(RCLCons(5, RefCell::new(Rc::new(RCLNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(RCLCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn really_dumb_tree() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn main() {
    println!("box_example\n");
    box_example();
    println!("\nderef_trait\n");
    deref_trait();
    println!("\ndrop_trait\n");
    drop_trait();
    println!("\nreferance_counter\n");
    referance_counter();
    println!("\nref_cycle_overflow\n");
    ref_cycle_overflow();
    println!("\nreally_dumb_tree\n");
    really_dumb_tree();
}
