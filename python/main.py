from dataclasses import dataclass
import csv
from enum import Enum
import random

class Category(Enum):
    Unit = 'Unit'
    Atomic = 'Atomic'
    Time = 'Time'
    People = 'People'
    Places = 'Places'
    Verb = 'Verb'
    Adjadv = 'Adjadv'
    Color = 'Color'
    Direction = 'Direction'
    Nature = 'Nature'
    Food = 'Food'
    Body = 'Body'
    Home = 'Home'
    Intangible = 'Intangible'
    Activity = 'Activity'
    Wearables = 'Wearables'
    Manmade = 'Manmade'
    Stationery = 'Stationery'
    Transport = 'Transport'
    Sentence = 'Sentence'

@dataclass(slots=True)
class Vocab:
    order: int
    hiragana: str
    kanji: str | None
    meaning: str
    category: Category
    example: str | None
    romanji: str

    def __repr__(self) -> str:
        return f'[{self.category.value}] | {self.order} | {self.hiragana} | ' \
               f'{self.kanji if self.kanji else ""} | {self.romanji}'

def start_showing(cats: list[Category]):
    print('Reading in vocab database...')

    # read
    vocabs: list[Vocab] = []
    with open('./data/words.csv', 'r') as file:
        my_reader = csv.reader(file, delimiter=',')
        for idx, row in enumerate(my_reader):
            if idx == 0:
                continue
            row[4] = Category(row[4].capitalize())
            vocabs.append(Vocab(*row))
    vocabs = [v for v in vocabs if v.category in cats]
    print(random.choice(vocabs))

def main():
    category = ['time', 'manmade']

    if not category:
        print('select category')
        category = [input()]

    print(f'Selected: {category}')
    cats = [Category(c.capitalize()) for c in category]
    start_showing(cats)


if __name__ == '__main__':
    main()
