mod lem;
use std::io;

fn main() {
    let bobo: String = lem::bobo("Gay");
    let mut name: String = String::new();
    name.push_str("Olabode Azeez");
    
    let uint8arr: [u8; 6] = [1,2,3,5,6,7];
    let slice: &[u8] = &uint8arr[1..3];
    let mut sum: u8 = 0;
    
    for i in slice.into_iter() {
        sum += i;
    }
    
    for l in 0..10 {
        println!("{} for iter", l)
    }
    println!("The sum of {:?} - {}", slice, sum);
    let that_new_shop = lem::Shop::new(name, 30);
    println!("Hello, world! {}, Shop name - {};", bobo, that_new_shop.name);

    let quiz: Vec<Vec<&str>> = vec![vec!["What is your Name?", "azeez", "jide", "azeez"], vec!["Which country are you from?", "Nigeria", "Ghana", "Nigeria"]];
    let mut total_scores: u8 = 0;

    for each_q in &quiz {
        println!("{}", each_q[0]);
        let mut ans = String::new();
        
        for option in &each_q[1..each_q.len() - 1] {
            println!("{}", option);
        }
        io::stdin().read_line(&mut ans).expect("Failed to read line");
        
        let correct_answer = String::from(each_q[each_q.len() - 1]);
        // let matcher: &str = each_q[each_q.len() - 2];
        println!("{} ---> len {} {} --len {} bool {} ", correct_answer, correct_answer.len(), ans.len(), correct_answer == ans, &ans);
        
        if  correct_answer == ans.trim() {
            println!("Correct!");
            total_scores += 1;
        }
    }

    println!("{:?}, Total scores - {}", quiz, total_scores);

}