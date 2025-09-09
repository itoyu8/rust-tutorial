pub fn run() {
    println!("Hello, world!"); //文字列を出力する場合

    print!("Hello, "); //print!だと改行されない
    print!("Rust!");

    println!("Hello,{}", "students"); //pythonと同じようにプレースホルダーが使用できる

    /*
    この間に記載した内容はコメントアウトされる
    */

    let a: i32 = 1; //変数に値を入れることを「束縛する」「バインドする」と呼ぶ
    println!("{}", a); //変数に対して値が束縛されていることが確認できる

    // a = 2; 変数に数値を上書きしようとするとエラー

    let mut b: i32 = 2;
    b = 3;

    println!("the value of b is: {}", b); // 変数を使用するとエラーが出なくなる

    let d: i32; 
    d = 1; // 先に型を指定してあとから変数の上書きが可能

    let d: &str = "test"; //変数名の上書き（シャドーイング）が可能

    const A: i32 = 1; //定数は必ず型が必要，定数はすべて大文字で書く
    const B: i32 = 2; //定数は関数の外でもglobalに定義できる
    // let C = 1; //letはglobalには定義できないのでエラーになる

    /* rustにおける数値型
    i（符号付き整数）：8,16,32,64,128,size：アルファベットと型をあわせて使用
    u（符号なし整数）：8,16,32,64,128,size
    f（浮動小数点）：32,64 */

    let a = 1; //勝手に推論される
    let b = 2.0;
    let c: u16 = 3;
    let d: f32 = 4.0f32; //let d: f32 = 4.0;だと浮動小数点はf64と推論されるのでエラーになる

    let e: i32 = 1 + 2; //
    let e: i32 = 1 - 2; //
    let e: i32 = 1 * 2; //
    let e: i32 = 1 / 2; //
    let e: i32 = 1 % 2; //

    let f = 1 as f64 + 2.0;  //整数型から浮動小数点型への変更でようやく足し算が可能

    /*
    論理型はtrue, falseのみ */
    let g: bool = false;

    let h: bool = 1 == 2; // Rustの補完機能が使えるが，そうじゃなくてもcargo runの時点で推論できる
    let h: bool = 1 != 2;
    let h = 1 > 2;
    let h: bool = 1 < 2;
    let h: bool = 1 >= 2;
    let h: bool = 1 <= 2;

    let t1 = (1, true, 2.0); // タプルの生成
    let t2 = (2.0, 1, true);
    println!("{:?}",t1); // タプルをprintするにはデバッグ表示として{:?}が必要

    let i = t1.0; // タプルのうち一部の要素のみを抽出する場合はl1.0
    println!("{}",i);
    
    let (x, y, _) = t2; // 必要のない変数はアンダースコアで

    let u: () = (); //中身のない空のタプルをユニット型と呼ぶ

    let l1 = [1, 2, 3]; // 配列はすべて同じ型でないといけない．タプルと違って異なる型は入れられない
    let l2 = [0;1000]; //[a;b]でaを1000個

    println!("{:?}", l1);

    let i = l1[0]; //配列から特定の要素を抜き出す場合はl1[0] ※タプルと違う
    
    let [x,y,z] = l1;

    let l3 = &l1[0..2]; //配列からのスライスは&l1[start..end]でendを含めるときは[start..=end]
    // [..a]なら最初からaまで，[a..]ならaから最後まで
    // &の説明はまた今度

    println!("{:?}",l3);

    let v1 = vec![1, 2, 3]; //vec!のあとで[]で使う．vecもすべて同じ型が必要
    let v2 = vec![0; 1000];
    let mut v3= Vec::new(); //空のベクタを作る場合はこれ．何かしら要素を入れないとエラー
    v3.push(1);
    v3.push(2);
    v3.push(3);
    println!("{:?}", v3);
    let x = v3.pop(); //要素を引く
    println!("{:?}",v3);

    let y = v3[1];
    let z = v3.get(100); //値が存在すればSome(T)，存在しなければNoneとなる
    println!("{:?}",z);

    let s = &v3[0..2]; //これも&が必要…まだよくわからない

    //文字型
    let c1 = 'a'; //charは'~'で囲む
    let c2 = '@';

    //文字列型
    let s1: &str = "Rust"; //文字列リテラル．実行時に値が確定している場合あとから変更できない．
    let s2 = String::from("Python"); //途中で文字列を変更したい場合はstring型．1つはString~fromで生成する
    let s3 = "Java".to_string(); //文字列スライスからstring型に直接変換

    let mut s4 = String::from("Hello");
    s4.push_str(", Rust"); //string
    println!("{}", s4);

    println!("{}", s4 + ", golang"); //stringと文字列リテラルは結合できる
    // println!("{}", s1 + ", golang"); //これはできない．

    let s5 = format!("{}{}", s1, s2); //formatだと問題なく結合できる










}

pub fn say_hello(){
    println!("Hello!");


    // let p = basics::add(1, 2);
    // println!("{}", p); こんな感じでmain()関数で呼ぶ

}

pub fn add(a: i32, b: i32) -> i32 {
    a + b // セミコロンなしで書くとそれが返り値になる

    // let d = basics::say_hello(); // ユニット型となる


}