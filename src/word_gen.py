class SimilarWordGenerator:
    """
    単語の文字を置換して、似た単語を生成するクラス

    誤字の修正に使う……と思う
    """

    def __init__(self) -> None:
        pass

    def replace1(self, word: str) -> list[str]:
        """
        1文字置換した単語を生成する

        置換する文字は
        https://www.bunka.go.jp/kokugo_nihongo/sisaku/joho/joho/kijun/naikaku/gendaikana/honbun_dai1.html
        にて定義されている、70個直音を対象とする

        つまり 70 ✕ 文字数 だけ単語が生成される

        例:
            あめ ->
                ああ, あい, あう, あえ, あお, あか ...
                あめ, いめ, うめ, えめ, おめ, かめ ...
        """
        # TODO: 実装する
        return []
