

fn main(){

}
trait areable {
     fn getArea(&self);
}

struct Circle {
    r:f32
}
struct Square {
    edge:f32
}
struct Triangle{
    bottle: f32,
    h:f32
}
impl areable for Circle {
    fn getArea(&self){
        
        println!("{}",self.r*3.14*3.14);
    }
}
impl areable for Square {
    fn getArea(&self){
        
        println!("{}",self.edge*self.edge);
    }
}
impl areable for Triangle {
    fn getArea(&self){
        
        println!("{}",0.5*self.bottle*self.h);
    }
}