use kana2kanzi::bi_gram_conv::*;
use std::io::{self, Write as _};

fn main() {
    // DB初期化はこれをコメントアウトする
    // use kana2kanzi::{bigram_db::BigramDB, dict_db::DictDB};
    // DictDB::build();
    // BigramDB::build();

    println!("辞書ロード中……");
    let conv = Kana2kanziConverter::new();
    println!("辞書ロード完了");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();
        if input == "exit" {
            break;
        }

        let kanzi = kana2kanzi_with_typo(&conv, input);
        println!("{}", kanzi);
    }
}
