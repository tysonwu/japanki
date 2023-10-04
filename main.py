import os

import pandas as pd
import cutlet

# df = pd.read_csv(f'{os.path.dirname(__file__)}/data/words.csv', index_col='order')
# df[['meaning', 'example']] = df['examples'].str.rsplit("\n", n=1, expand=True)
# df = df[['hiragana', 'kanji', 'meaning', 'example']]
# df.to_csv(f'{os.path.dirname(__file__)}/data/words2.csv', index=False)

# katsu = cutlet.Cutlet(use_foreign_spelling=False, ensure_ascii=False)

# df = pd.read_csv(f'{os.path.dirname(__file__)}/data/words.csv')
# df['romaji'] = df['hiragana'].apply(lambda w: katsu.romaji(w))
# df.to_csv(f'{os.path.dirname(__file__)}/data/words2.csv', index=False)

df = pd.read_csv(f'{os.path.dirname(__file__)}/data/words.csv')
df.index.name = 'order'
df = df.reset_index()
# print(df['category'].unique())
df.to_csv(f'{os.path.dirname(__file__)}/data/words2.csv', index=False)
