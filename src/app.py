from pathlib import Path
import sys

from converter import KanaKanjiConverter
from dictionary import KanaKanjiDictionary
from word_gen import SimilarWordGenerator

if __name__ == "__main__":
    converter = KanaKanjiConverter(
        generator=SimilarWordGenerator(), dictionary=KanaKanjiDictionary(Path("dict"))
    )

    if len(sys.argv) < 2:
        print("文章を入力してください")
        sys.exit(1)

    text = sys.argv[1]

    converted_text = converter.kana_to_kanji(text)

    print("変換結果:", converted_text)
