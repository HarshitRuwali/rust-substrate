fn main() {
    println!("Hello, world!");
    
    let x = 5;
    println!("{}", x);
    
    //tuple
    let bootcamp_det: (&str, u64) = ("substrate", 090722);

    println!("{}", bootcamp_det.0);
    println!("{}", bootcamp_det.1);

    let arr = [1, 2, 3, 4];
    println!("{}", arr[0]);
    
    println!("{}", new_function(43));
    let tmp = new_function(32);

    println!("is tmp less then 32 ? {}", compare(tmp));
    
    let mut count = 0;

    loop {
        println!("yeh, memory alloc is happening, ram will be exhausted and your system will crash");
        count+=1;

        if count >= 5{
            break;
        }
    }

    let mut count = 0;
    
    let res = loop {
        println!("yeh, memory alloc is happening, ram will be exhausted and your system will crash");
        count+=1;

        if count >= 5{
            break count * 2;
            // break;
        }
    };

    println!("{}", res);

    let mut count = 5;

    while count != 0{
        println!("{}", count);
        count-=1;
    }

    // for-in loop

    let arr = [1, 2, 3, 4];

    for i in arr.iter(){
        println!("arr element is {}", i);
    }

    
    for i in (1..5){
        println!("{}", i);
    }

    // revese 
    for i in (1..5).rev(){
        println!("{}", i);
    }

}

fn new_function(x:i32) -> i32{

    println!("new function called");
    println!("x is {}", x);

    let mul = x*5;
    // println!("{}", mul);

    return mul;
}

fn compare(x:i32) -> bool{
    if x < 32{
        println!("no is less then 32");
        return true;
    }else{
        return false;
    }
}