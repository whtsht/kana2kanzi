from word_gen import SimilarWordGenerator
from dictionary import KanaKanjiDictionary, KanaKanjiDictionaryMock


class KanaKanjiConverter:
    """
    かな漢字変換を行うクラス
    """

    def __init__(
        self,
        generator: SimilarWordGenerator,
        dictionary: KanaKanjiDictionary | KanaKanjiDictionaryMock,
    ) -> None:
        self.generator = generator
        self.dictionary = dictionary

    def kana_to_kanji(self, text: str) -> str:
        """
        分割数最小法で、かな漢字変換を行う

        手順:
            1. かな文字列を1~4-gramで分割する
            2. 分割数が最小になるように、分割した文字列を選ぶ
            3. かな文字列を漢字に変換する

        文字を結合できる条件:
            かな文字列が辞書に登録されている（dictionary.get(<かな文字列>)がNoneでない）

        例:
            わたしはりんごがすきだ

            1~4-gramで分割する
            わ　た　し　は　り　ん　ご　が　す　き　だ
            わた　たし　しは　はり　りん　んご　ごが　がす　すき　きだ
            わたし　たしは　しはり　はりん　りんご　んごが　ごがす　...
            わたしは　たしはり　しはりん　はりんご　りんごが　んごがす ...

            分割数が最小になるように、文字を分割する
            わたし　は　りんご　が　すき　だ

            漢字に変換する
            私　は　林檎　が　好き　だ

        愚直な実装だと計算量はO(N^4)となるはず
        別に4という数にこだわる必要はないが、計算量が爆発的に増えるので
        """

        # TODO: 実装する
        return text
