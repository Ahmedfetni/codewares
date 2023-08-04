

  // String slicing into two pairs in the case of odd number of char s than we have to add _ to the last pair 
  
  fn solution(s:&str) -> Vec<String>{
    let mut vec:Vec<String>= Vec::new();
    let end:usize= s.len() - s.len() % 2;

    
    
    for  i in (0..end).step_by(2){
      
      let second_of_pair = i+2;
        if second_of_pair <= end {
          vec.push(s[i..second_of_pair].to_string());
        } 
    
    }

    if end != s.len(){
      let  last :String;       
      match s.chars().last(){
        None=> return vec,
        Some(v) =>  { last = format!("{}{}",v,"_");
        vec.push(last);}
      }
    }
    return vec;
}

use itertools::Itertools;

fn clever_solution(s: &str) -> Vec<String> {
    s.chars().chunks(2).into_iter().map(|c| c.pad_using(2, |_| '_').collect()).collect()

    //converting chars to chunks that turing that to an iterator than mapping each one to padding thats add "_"
}



//converting to camel case 
fn to_camel_case(text:&str)-> String {
  let mut words :String =text
                      .split('_')
                      .flat_map(|c| c.split('-'))
                      .collect::<Vec<&str>>()
                      .into_iter()
                      .map(|s| s.to_string())
                      .collect::<Vec<String>>()
                      .into_iter()
                      .enumerate()
                      .map(|(i,s)| if i != 0 { match s.chars().next(){
                      Some(first_char) =>{
                            format!("{}{}",first_char.to_ascii_uppercase(),&s[1..])
                      },
                      None => s,
                    }}else{ s}).collect::<Vec<String>>().join(""); 
  
                    return words;
}
fn clever_to_camel_case(text: &str) -> String {
  text.split(&['-', '_'])
      .enumerate()
      .map(|(i, w)| match i {
          0 => w.to_string(),
          _ => w[..1].to_uppercase() + &w[1..],
      })
      .collect() // we don't need to add a joint collect create a string out of it cause the return type is inferred on it
}

/* 
// reverse words that has 5 or more chars 
fn spin_words(words: &str) -> String {
  words.split(' ')
       .map(|s| if s.len()>=5 {
        s.chars()
        .rev()
        .collect::<String>()
      } else {s.to_string()} )
      .collect::<Vec<String>>()
      .join(" ")
}*/


/*fn count_duplicates(text: &str) -> u32 {
    
}*/
fn main() {
  let chaine =  "aA11";
   

}
