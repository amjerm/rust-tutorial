fn main() {
    let v = vec![1, 2, 3, 4];
    println!("The vector is {:?}", v);
    
    let mut second: Vec<i32> = Vec::new();
    println!("The vector is {:?}", second);

    second.push(33);
    println!("The vector is {:?}", second);
    
    let second_v: &i32 = &v[1];
    println!("The second and third value of v are {} and {}", second_v, &v[2]);
    
    match v.get(2) {
        Some(value) => println!("The second value of v is {}", value),
        None => println!("There is no second value of v"),
    }

    match v.get(9) {
        Some(value) => println!("The ninth value of v is {}", value),
        None => println!("There is no ninth value of v"),
    }
}
