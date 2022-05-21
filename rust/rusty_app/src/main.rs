// use rand::Rng;
fn main() {
    // let x: i32 = -3;
    // println!("Hello, world! {}", x);

    my_fun_2();
    my_fun_3("Fadhelah".to_string());
    my_fun_4();
    my_fun_5();
    random_shit();
    else_else();
}

//Functions
fn my_fun_2(){
    println!("Hello Reagan")
}

fn my_fun_3(name :String){
        println!("Hello {}", name);
}

fn my_fun_4() -> i32{
    return -33;
}

fn my_fun_5() ->f32{
    30.3
}

fn random_shit(){
    let mut my_number: i32 = 3;
    loop{
        println!("Again! number {}", my_number);
        if my_number == 5{
            break;
        }
        my_number +=1;
    }
}

//if else
fn else_else(){
    let my_number2: i32 = 4;
    if my_number2 == 3{
        println!("my number is {}", my_number2);
    }
    else{
        println!("Not equal");
    }
}

//ownership
fn borrower(){
    let mut s:String = String::from("Hello")

    // let r1:&mut String = &s;
    // let r2:&mut String = &s;
}