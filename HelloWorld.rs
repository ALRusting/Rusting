fn hw_func(arg1: &str){
  println!("Hello, {arg1}");
}

fn main(){
  let mut var="World";
  hw_func(var);
  var="Luca";
  hw_func(var);
}
