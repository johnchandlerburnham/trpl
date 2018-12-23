#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
  {
    let mut s = String::new();

    // the following are all equivalent
    let data = "initial contents";
    let s = data.to_string();

    let s = "initital contents".to_string();

    let s = String::from("inital contents");
  }

  {
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
  }

  {
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);
  }

  {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1);
    println!("s2 is {}", s2);
  }

  {
    let mut s = String::from("lo");
    s.push('l');
  }

  {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    println!("s3 is {}", s3);
  }

  {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s);
  }

  {
    let s1 = String::from("hello");
    //  let h = s1[0];

    let len = String::from("Hola").len();
    println!("len of {} is {}", "Hola", len);

    let len = String::from("Здравствуйте").len();
    println!("len of {} is {}", "Здравствуйте", len);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s is {}", s);

    for b in hello.bytes() {
      println!("{}", b);
    }
  }
}