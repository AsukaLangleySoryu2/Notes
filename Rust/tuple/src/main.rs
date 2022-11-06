struct User{
        active: bool,
        username:String,
        email:String,
        sign_in_count:u64,
    }
struct File{
        name:String,
        data:Vec<u8>,
    }

struct AlwaysEqual;
/* impl SomeTrait for AlwaysEqual{

   } */
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let tup:(i32,f64,u8)=(500,6.4,1);
    let (x,y,z)=tup;
    println!("The value of  y is:{}",y);
    let five_hundred=tup.0;
    let six_point_four=tup.1;
    let one=tup.2;
    println!("The value {},{},{}",five_hundred,six_point_four,one);
    let s1=String::from("Hello, world!");
    let s2=calculate_length(s1);
    println!("{},{}",s2.0,s2.1);

    
    let mut user1=User{
        active:true,
        email:String::from("zh1610596998@gmail.com"),
        sign_in_count:1,
        username:String::from("大猫221"),
    };
    user1.email=String::from("anotheremail:1610596998@qq.com");
    let user2=User{
        email:String::from("anotheremail:1610596998@qq.com"),
        ..user1
    };
    println!("{}",user1.email);

    let f1 = File {
     name: String::from("f1.txt"),
     data: Vec::new(),
   };
   let f1_name = &f1.name;
   let f1_length = &f1.data.len();
//    println!("{:?}",f1);
   println!("{} is {} bytes long", f1_name, f1_length);

   struct Color(i32,i32,i32);
   struct Point(i32,i32,i32);
   let black=Color(0,0,0);
   let origin=Point(0,0,0);

   
//    let sunject=AlwaysEqual;
   //结构体数据，所有权
   let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect1);
    println!("rect1 is {:?}", rect1);

}
fn build_user(email: String, username: String) -> User{
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
fn calculate_length(s:String)->(String,usize){
    let size=s.len();
    (s,size)
}