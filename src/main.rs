use kana2kanzi::bi_gram_conv::Kana2kanziConverter;

fn main() {
    let conv = kana2kanzi::bi_gram_conv::Kana2kanziConverter::new();
    kana2kanzi(&conv, "きょうはいいてんき");
    kana2kanzi(&conv, "あめがやむまでまつ");
    kana2kanzi(&conv, "くるまででかける");
    kana2kanzi(&conv, "かれがくるまでまつ");
    kana2kanzi(&conv, "かなかんじへんかんはむずかしい");
}

fn kana2kanzi(conv: &Kana2kanziConverter, input: &str) {
    let kanzi = kana2kanzi::bi_gram_conv::kana2kanzi(conv, input);
    println!("{}\n    -> {}", input, kanzi);
}
