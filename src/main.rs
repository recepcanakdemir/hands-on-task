fn main() {
    let s1  = String::from("Hello");
    let len = calculate_length(&s1);
    
    println!("The length of the {} is {}",s1,len);



    let mut a1 = String::from("Hello");

    let r1 = &a1;
    let r2 = &a1;
    println!("{},  {}",r1,r2);

    let r3 = &mut a1;
    println!("{}",r3);
}

fn calculate_length(s: &String) -> usize{
     s.len()
}
