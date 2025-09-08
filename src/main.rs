fn main() {
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



}
