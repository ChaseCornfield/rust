fn main ()
{
   // Definition for singly-linked list.
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

struct Solution{}
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> 
    {
        // s oto make this work we have to grab one list and for each item we compare with the other list and go popping
        // unitl one of the lists is done, when one is done we just copy and paste the remainig list on the output list

        let result: ListNode = ListNode::new(4);
        for num in list1{

            for num2 in list2{
                if num.val <= num2.val
                {
                    
                }
            }
        }
        Some(_)
    }   
}
}