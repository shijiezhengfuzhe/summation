fn main()
{let one =Some(1i32 as u32) ;
    let infinite =one.map(|i| (i+100)*100/2);
    println!("infinite: {:?}",infinite);
    let none:Option<u32>=None;
    let none_result=none.map(|i| i+1);
}