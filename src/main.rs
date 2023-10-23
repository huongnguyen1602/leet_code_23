#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}




fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => return None,
        (Some(n1), None) => return Some(n1),
        (None, Some(n2)) => return Some(n2),
        (Some(n1), Some(n2)) => {
            let sum = n1.val + n2.val;
            if sum < 10 {
                return Some(Box::new(ListNode {
                    val: sum,
                    next: add_two_numbers(n1.next, n2.next)
                }))
            } else{
                let carry = Some(Box::new(ListNode::new(1)));
                return Some(Box::new(ListNode {
                    val: sum - 10,
                    next: add_two_numbers(add_two_numbers(carry, n1.next), n2.next)
                }))
            }
        }        
    }   
}


fn main() {
    ////understand what is listnode
    // let a = ListNode::new(3);
    // let b = ListNode{
    //     val: 2,
    //     next: None,
    // };
    // let mut c = ListNode{
    //     val: 1,
    //     next: Some(Box::new(a.clone())),
    // };
    // println!("{:?}", a);
    // println!("{:?}", b);
    // println!("{:?}", c);
    // println!("{:?}", c.next.unwrap().val);
    // [2,4,3]
    let list_one = Some(
        Box::new(ListNode{
            val: 2,
            next: Some(Box::new(ListNode{
                val: 4,
                next: Some(Box::new(ListNode{
                    val: 3,
                    next: None,
                }))
            }))
        })
    );
    // [5,6,4]
    let list_two = Some(Box::new(ListNode{
        val: 5,
        next: Some(Box::new(ListNode{
            val: 6,
            next: Some(Box::new(ListNode{
                val: 4,
                next: None,
            }))
        }))
    }));
    let result = add_two_numbers(list_one, list_two);
    println!("{:?}", result);
}