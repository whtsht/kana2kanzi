import unittest
from dictionary import KanaKanjiDictionaryMock
from word_gen import SimilarWordGenerator
from converter import KanaKanjiConverter


mock_dict = KanaKanjiDictionaryMock(
    {
        "わた": "綿",
        "たし": "足し",
        "りん": "輪",
        "がす": "ガス",
        "すき": "好き",
        "わたし": "私",
        "りんご": "林檎",
    }
)

generator = SimilarWordGenerator()
converter = KanaKanjiConverter(generator, mock_dict)


class TestKanaKanjiConverter(unittest.TestCase):

    def test_かな漢字変換_タイプミスなし(self):
        self.assertEqual(
            converter.kana_to_kanji("わたしはりんごがすきだ"), "私は林檎が好きだ"
        )

        self.assertEqual(
            converter.kana_to_kanji("りんごがわたしはすきだ"), "林檎が私は好きだ"
        )

        # TODO: テストケースを追加する


if __name__ == "__main__":
    unittest.main()
