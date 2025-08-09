#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Debug + std::fmt::Display> List<T> {
    pub fn new() -> List<T> {
        List{head:None}
    }

    pub fn push(&mut self, value: T) {
    //    println!("{:?} {}",self.head,value);
       let nod=Node{
        value,
       next:self.head.take().map(Box::new),
       };
       self.head =Some(nod);
    //    println!("{:?} {}",self.head,&value);

        // self.head.push(Node{
        //     value:Some(value),
        //     next:Some(Box::new(Node{}))
        // });
    }

    pub fn pop(&mut self) {
         if let Some(node) = self.head.take() {
        self.head = node.next.map(|b| *b);
    }
    }

    pub fn len(&self) -> usize {
        let mut  res:usize =0;
        let mut head=self.head.as_ref();
        while let Some(n)=head{
            // println!("{} *",n.next.iter().count());
            res+=1;
           head=n.next.as_deref();
        }
     res
        // self.head.len()
    }
}