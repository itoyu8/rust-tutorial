// バイナリクレートはmain()を含む必要がある
//rust-tutorial は，main.rsをバイナリクレートとして認識する
// この場合，cargoに対してどれを実行するかを指定する

// cargo run --bin rust-tutorial で，main.rsが動く
// cargo run --bin bin1 で，bin/bin1.rsが動く

fn main() {
    println!("this is bin1");
}