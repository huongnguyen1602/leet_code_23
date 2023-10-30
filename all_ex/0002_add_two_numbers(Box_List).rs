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
  

  fn create_list_node_from_vec(vec: &mut Vec<i32>) -> Option<Box<ListNode>>{
    let mut result = Box::new(ListNode::new(0));
    let mut ref_result = &mut result;
    vec.reverse();
    while vec.len() != 0 {
        let val = vec.pop().unwrap();
        ref_result.next = Some(Box::new(ListNode::new(val)));
        ref_result = ref_result.next.as_mut().unwrap();
    }
    result.next
  }
}




// fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     match (l1, l2) {
//         (None, None) => return None,
//         (Some(n1), None) => return Some(n1),
//         (None, Some(n2)) => return Some(n2),
//         (Some(n1), Some(n2)) => {
//             let sum = n1.val + n2.val;
//             if sum < 10 {
//                 return Some(Box::new(ListNode {
//                     val: sum,
//                     next: add_two_numbers(n1.next, n2.next)
//                 }))
//             } else{
//                 let carry = Some(Box::new(ListNode::new(1)));
//                 return Some(Box::new(ListNode {
//                     val: sum - 10,
//                     next: add_two_numbers(add_two_numbers(carry, n1.next), n2.next)
//                 }))
//             }
//         }        
//     }   
// }


fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut head = Box::new(ListNode::new(0));
    let mut tail = &mut head;
    while l1 != None || l2 != None || carry != 0 {
        let val = match l1 {
            Some(n) => {l1 = n.next; n.val},
            None => 0,
        } + match l2 {
            Some(n) => {l2 = n.next; n.val},
            None => 0
        } + carry;
        carry = val/10;
        tail.next = Some(Box::new(ListNode::new(val%10)));
        tail = tail.next.as_mut().unwrap();
    }  
    return head.next;
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
    let mut vec = vec![1,2,3];
    let d = ListNode::create_list_node_from_vec(&mut vec); 
    println!("{:?}", d);
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