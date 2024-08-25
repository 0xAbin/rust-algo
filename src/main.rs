fn main() {

//     // ----Numbers
//     let x: i8 = 5;
//     let y: i8 = 5;
//     let z = x + y;
//     println!("total : {}", z) ;

    
// //    ------Bool
//    let mut test: bool = false;

//    test = true;

//    if test {
//        println!("Test is true");
//    } else {
      
//      println!("Test is false");
//    }


//  // --- Strings  
//   let string  = String::from("String Test");
//   println!("{}", string);


  // for i in 0..10 {
  //   println!("Number is {}", i);
  // }


   let strings = String::from("testing");


   println!("{}", strings);


   for strings in strings.split_whitespace() {
       println!("{}", strings);
   }
     


}


fn testing(words: String) -> String {

   let  mut ans  : String = String::from("");
    
    for char in words.chars() {
       ans.push_str(&char.to_string());
       if char == ' ' {
        break;
       }
    }

return ans;

}



 