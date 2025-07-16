fn main(){
    let mut vector=vec![1,2,3,4,5];
    let element=&vector[0];
    vector.push(1);
    println!("{element}");
}