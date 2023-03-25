fn main(){
 let x :[u8;3]=[2,253,2];
 let sum =add(&x);
 if let None =sum{
    println!("Error");
 }else {
     println!("{}",sum.unwrap() );
 }

}
fn add(seq :&[u8])->Option<u8>{
    let mut sum =0;
    for i in seq{
        if sum >u8::MAX-i{
            return None;
        }
        sum+=i;
    }
    
    Some(sum)
}