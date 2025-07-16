struct Linspace {
    a:usize,
    b:usize,
}


impl Iterator for Linspace{
    type Item=usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.a+=1; 
        if self.a==self.b {
            return None;
        }
        else {
            return Some(self.a);
        }


    }
}

fn main(){
    let our_own_iterator=Linspace {a:10,b:15};
    for x in our_own_iterator{
        println!("{x}");
    }


}

