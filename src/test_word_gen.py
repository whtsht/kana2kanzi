import unittest
from word_gen import SimilarWordGenerator

generator = SimilarWordGenerator()


class TestSimilarWordGenerator(unittest.TestCase):

    def test_正しい単語を含む(self):
        self.assertIn("そら", generator.replace1("そた"))
        self.assertIn("あめ", generator.replace1("あべ"))

        self.assertIn("りんご", generator.replace1("りあご"))
        self.assertIn("たまご", generator.replace1("らまご"))

        self.assertIn("ともだち", generator.replace1("ともたち"))
        self.assertIn("うんどう", generator.replace1("うんとう"))

        # TODO: テストケースを追加する

    def test_生成される単語数(self):
        self.assertEqual(70, len(generator.replace1("あ")))
        self.assertEqual(70 * 2, len(generator.replace1("あい")))
        self.assertEqual(70 * 3, len(generator.replace1("あいう")))
        self.assertEqual(70 * 4, len(generator.replace1("あいうえ")))


if __name__ == "__main__":
    unittest.main()
