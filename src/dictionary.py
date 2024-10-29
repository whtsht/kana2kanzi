from pathlib import Path
from typing import Dict


class KanaKanjiDictionary:
    """
    かな <-> 漢字のペアを保持するクラス
    """

    def __init__(self, root_path: Path) -> None:
        if not root_path.is_dir():
            raise ValueError(f"{root_path} is not a directory")

        self.root_path = root_path

    def get(self, key: str) -> str | None:
        """
        かなをキーとして、漢字を取得する
        対応する漢字が存在しない場合はNoneを返す
        """
        # TODO: 実装する
        return None


class KanaKanjiDictionaryMock:
    """
    KanaKanjiDictionaryのモッククラス
    """

    def __init__(self, dict: Dict[str, str]) -> None:
        self.dict = dict

    def get(self, key: str) -> str | None:
        return self.dict.get(key)
