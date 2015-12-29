use std::rc::Rc;
use std::mem;
use std::collections;
extern crate bytes;

#[macro_use]
extern crate nom;

#[allow(dead_code)]
enum Value {
    Nil,
    True,
    False,
    Integer(i64),
    Rational(i64, i64),
    String(Rc<String>),
    Buffer(Rc<bytes::Buf>),
    Pair(Rc<(Value, Value)>),
    BTreeMap(Rc<collections::BTreeMap<Value, Value>>),
    BTreeSet(Rc<collections::BTreeSet<Value>>),
    BinaryHeap(Rc<collections::BinaryHeap<Value>>),
    HashMap(Rc<collections::HashMap<Value, Value>>),
    HashSet(Rc<collections::HashSet<Value>>),
    LinkedList(Rc<collections::LinkedList<Value>>),
    VecDeque(Rc<collections::VecDeque<Value>>),
}

fn main() {
    println!("Value is {} bytes wide.", mem::size_of::<Value>());
}
