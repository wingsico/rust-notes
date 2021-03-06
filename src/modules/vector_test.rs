pub fn start() {
  let s = String::from("1");
  let mut v = vec![s, String::from("2")];

  // 取数方式1

  v.push(String::from("4"));

  let first_number_2 = v.get(0);

  if let Some(d) = first_number_2 {
    println!("{}", d);
  }

  let string1 = "string";
  let string2 = "string2";

  {
    let mut str_vec = vec![string1, string2];

    let a = str_vec[0];
    str_vec.push("string");
    println!("{}", a);
  }

  println!("{}", string2);

  let string3 = String::from("hahah");
  let string4 = String::from("ggag");
  let string5 = String::from("sgasgd");

  {
    let mut str_vec = vec![string3, string4];

    let a = &str_vec[0];
    println!("{}", a);
    str_vec.push(string5);
  }

  // now string3, string4 and string5 had dropped...
  // println!("{}", string3); painc

  // loop for vec

  let mut vec3 = vec![1, 2, 3, 4, 5];

  for i in &mut vec3 {
    *i += 30;
  }

  println!("{:#?}", vec3);


  // borrow here for loop
  let string6 = String::from("hahah");
  let string7 = String::from("ggag");

  let mut vec4 = vec![string6, string7];

  for i in &mut vec4 {
    i.make_ascii_uppercase();
  }
  println!("{:#?}", vec4);


  // diffrent type vec
  #[derive(Debug)]
  enum TableRowValue {
    Float(f32),
    Text(String),
    Int(i32),
  }

  let vec5: Vec<TableRowValue> = vec![TableRowValue::Float(5.0), TableRowValue::Int(4), TableRowValue::Text(String::from("hahahah"))];

  for i in vec5 {
    // 取值方式一
    if let TableRowValue::Float(f) = i {
      println!("{}", f);
    }

    // 取值方式二
    match i {
      TableRowValue::Float(f) => println!("{}", f),
      TableRowValue::Text(t) => println!("{}", t),
      TableRowValue::Int(i) => println!("{}", i),
    }
  }
}
