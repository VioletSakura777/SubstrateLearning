fn main(){
    let red =TrafficLight::Red;
    let green =TrafficLight::Green;
    let yellow =TrafficLight::Yellow;

   
    println!("{}",red.getTime());
    println!("{}",green.getTime());
    println!("{}",yellow.getTime());

}
enum TrafficLight {
    Red,Yellow,Green
} 
trait  Time {
    fn getTime(&self)->u32;
}

impl Time for TrafficLight {
    fn getTime(&self)->u32{
        match self {
            TrafficLight::Red =>{60}
            TrafficLight::Yellow =>{10}
            TrafficLight::Green =>{60}
        }
    }
}