struct Node {
    pub val: u32,
    pub children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: u32) -> Node {
        Node {
            val: val,
            children: vec![],
        }
    }
}
