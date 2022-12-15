// 第一题：为枚举通信号灯实际发现一个trait，trait里包含一个返回时间的方法，不同的灯持续的时间不同；
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
 
trait GetDuration { 
    fn get_duration(&self) -> u32; // 返回持续的时间（单位：秒） 
} 
 
impl GetDuration for TrafficLight { 
   fn get_duration(&self) -> u32 {       // 实现trait方法 
      match *self {                      // 根据枚举实例返回不同的值 
          TrafficLight::Red     => 30,   // 红灯持续30s 
          TrafficLight::Yellow  => 3,    // 黄灯持续3s 
          TrafficLight::Green   => 40,   // 绿灯持续40s     
      }  
   }    
}      

fn main() {     // 测试功能  

 let red = TrafficLight::Red;               // 定义变量  

 println!("红灯持续时长 : {}", red.get_duration());      

 let yellow = TrafficLight::Yellow;         // 定义变量  

 println!("黄灯持续时长 : {}", yellow.get_duration());    

 let green = TrafficLight::Green;           // 定义变量      

 println!("绿灯持续时长 : {}", green.get_duration());

}