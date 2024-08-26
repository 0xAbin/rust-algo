### Rust Algo


### Basic Hello world  
```rust
fn main() {
    println!("Hello, world!");
}
```


### Variables
```rust
  fn string() {
    let x: i64 = 100;
    let my_string = String::from("Testing variables : ");
    println!("{} {}", my_string, x);
  }
```

### Mutable 
```rust

  fn mutable() {
    let mut x: i64 = 200;
    x = 100;
    println!("{}", x);
  }
```


#### loops 

# for loop
```rust
 fn for_loop(){
    for i in 0..5 {
        println!("{}", i);
    }
 }
```
# if else
```rust
    fn if_else() {
    let yes:bool = false;
     if yes {
        println!("yes");
       }
    else {
        println!("no");
       }
  }
```
# while loop
```rust
  fn while_loop(){
    let mut x: i64 = 5;
    while x > 7 {
      print!("Big daddy dick heehheh");
      x -= 1;
    }
    println!("got small dick");
  }
```