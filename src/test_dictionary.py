from pathlib import Path
import unittest

from dictionary import KanaKanjiDictionary

dictionary = KanaKanjiDictionary(Path("dict"))


class TestKanaKanjiDictionary(unittest.TestCase):

    def test_辞書に単語がまれる場合はアクセスできる(self):
        self.assertEqual(dictionary.get("わたし"), "私")

    def test_辞書に単語がまれる場合はNoneを返す(self):
        self.assertIsNone(dictionary.get("わたしい"))


if __name__ == "__main__":
    unittest.main()
