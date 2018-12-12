extern crate regex;

struct Node {
    children: Vec<Node>,
    meta: Vec<i32>,
}

impl Node {
    fn new() -> Node {
        Node { children: Vec::new(), meta: Vec::new() }
    }

    fn parse(&mut self, tokens: &mut impl Iterator<Item = i32>)
    {
        let num_kids = tokens.next().unwrap();
        let num_meta = tokens.next().unwrap();

        for _i in 0..num_kids {
            let mut kid = Node::new();
            kid.parse(tokens);
            self.children.push(kid);
        }
        for _i in 0..num_meta {
            self.meta.push(tokens.next().unwrap());
        }
    }

    fn sum_meta(&self) -> i32 {
       let kids_sum : i32 = self.children.iter().map(|c| c.sum_meta()).sum();
       let self_sum : i32 = self.meta.iter().sum();
       kids_sum + self_sum
    }

    fn value(&self) -> i32 {
        let result = if self.children.len() == 0 {
            print!("leaf ");
            self.meta.iter().sum()
        } else {
            let mut sum = 0i32;
            for kid_idx in self.meta.iter() {
                if let Some(kid) = self.children.get((*kid_idx - 1) as usize) {
                    sum += kid.value();
                }
            }
            print!("parent ");
            sum
        };
        println!("node value {}", result);
        result
    }
}

fn main() {
    let input = include_str!("8_input");
    let re = regex::Regex::new(r"\d+").unwrap();
    let mut tokens = re.find_iter(input).map(|e| e.as_str().parse::<i32>().unwrap());
    let mut node = Node::new();
    
    node.parse(&mut tokens);
    println!("meta sum {}", node.sum_meta());
    println!("value {}", node.value());
}

