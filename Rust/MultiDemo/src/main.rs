fn main() {
    //复合类型
    /* let mut f1=File::from("./test.txt");
    open(&mut f1);
    close(&mut f1); */
    // let my_name = "Pascal";
    // greet(my_name);
    //字符串类型

    let s = String::from("Hello world!");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}\n{}", hello, world);
    let test = &s[..11];
    let test1 = &s[6..];
    println!("{}\t{}", test, test1);
    // let s = "中美人";
    // let a = &s[0..2];
    // println!("{}", a);
    //这是绑定了一个对象
    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear(); // error!这里是什么地方有问题。?
    //这里清楚了s但是引用还存在着，所以出现悬空指针

    // println!("the first word is: {}", word);

    //集合的切片
    let a=[12,3,4,6];
    let slice=&a[1..3];
    assert_eq!(slice,&[3,4]);

    //字符串str类型是硬编码类型，不可变
    let _a="helloRust".to_string();
    let mut b=String::from("Hello World");
    let _a=['न', 'म', 'स', '्', 'त', 'े'];
    b.push('r');
    b.push_str("Just for test");
    b.insert(5,',');
    b.insert_str(6,"My name is insert");
    println!("{}",b);
    let mut testPlus=b.replace("Hello","NotHello");
    println!("{}",testPlus);
    testPlus.pop();
    testPlus.remove(0);
    testPlus.truncate(5);
    testPlus.clear();
    let rust=String::from("Rust");
    let result=rust+&b;//这里就是将b的地址添加到a的后面，注意这里主字符串的所有权会被释放
    println!("{}",result);
    let s1 = "hello";
    let s2 = String::from("rust");
    let s = format!("{}test test {}!", s1, s2);
    println!("{}", s);
    //输出字符
    let byte_eacape="I'm writing \x52\x75\x73\x74";
    //双斜杠表示保持字符串原样
    println!("What are you doing\x3F(\\x3f means?){}",byte_eacape);
    //输出unicode字符
    let unicode_codepoint="\u{211D}";
    let character_name="\"DOUBLE-STRUCK \"";
    println!("Unicode character {} (U+211D) is called {}",
    unicode_codepoint,character_name);

    let long_string="String literals
                    can span multiple lines.
                    The linebreak and indentation here ->\
                    <- can be escaped too!";
    println!("{}",long_string);                
    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}",raw_str);
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}",quotes);
    for c in "中国人".chars(){
        println!("Hello,{}",c);
    }
    for b in "中国人".bytes(){
        println!("{}",b);
    }
    {
        let s =String::from("Hello");
    }



}
fn _greet(name: String) {
    println!("Hello, {}!", name);
}
fn _first_word(s:&String)->&str{
    &s[..1]
}
/* type File=String;
fn open(_f1:&mut File) -> bool{
    true
}
fn close(_f3:&mut File) -> bool{
    true
}
fn _read(_f4:&mut File,_save_to:&mut Vec<u8>)->!{
    unimplemented!();
} */
