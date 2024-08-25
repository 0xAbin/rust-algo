fn main() {

 let strings = String::from("testing space testing");


  //  println!("{}", strings);
  
  let format = testing(strings);

   println!("{}", format);

}


fn testing(words: String) -> String {

   let  mut ans  : String = String::from("");
    
    for char in words.chars() {
       ans.push_str(char.to_string().as_str());
       if char == ' ' {
        break;
       }
    }
   
  return ans;
}



 