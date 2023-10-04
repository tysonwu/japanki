import os

import pandas as pd

df = pd.read_csv(f'{os.path.dirname(__file__)}/data/words.csv', index_col='order')
df[['meaning', 'example']] = df['examples'].str.rsplit("\n", n=1, expand=True)
df = df[['hiragana', 'kanji', 'meaning', 'example']]
df.to_csv(f'{os.path.dirname(__file__)}/data/words2.csv', index=False)
