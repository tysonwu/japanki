<div style="text-align: center">
    <h1>JapAnki👹</h1>
    <h5>Learn Japanese N5 vocabularies and do quizzes on CLI!</h5>
</div>

## Installation

> To be written.

## Introduction

This simple CLI let's you study and memorize japanese N5 vocabs in terminal, like this:

```
>> japanki show all

Reading in vocab database...

====== Atomic 6 | Level 1 =====
～かた | ~ kata
Meaning: how to ...
Example: つかいかたを おしえてください。Please teach me how to use it.

> Show next ▷

====== Unit 1 | Level 1 =====
～えん | ～円 | ~ en
Meaning: ... yen
Example: これは 10,000円です。This is 10,000 yen.

> Show next ▷

====== Verb 54 | Level 1 =====
あけます | Akemasu
Meaning: open
Example: わたしは ドアをあけます。I open the door.

...

```

## Features and usage

### List catories

```
japanki list   # list all available categories

Available categories:
 - Unit
 - Atomic
 - Time
 - People
 - Places
 - Verb
 - Adjadv
 - Color
 - Direction
 - Nature
 - Food
 - Body
 - Home
 - Intangible
 - Activity
 - Wearables
 - Manmade
 - Stationery
 - Transport
 - Sentence
```

### Show Japanese vocabularies
```
japanki show all                                          # show vocabs in all categories
japanki show some verb time unit                          # show within the verb/time/unit category
japanki show some verb time unit --kanji                  # show vocabs that involves kanji
japanki show some verb time unit --kanji --no-progress    # no progress checks
```

### Quiz yourself by typing some romaji or vocabulary meanings

#### Example usage
```
japanki quiz all                                          # quiz with vocabs in all categories
japanki quiz some verb time unit                          # quiz within the verb/time/unit category
japanki quiz some verb time unit --kanji                  # quiz vocabs that involves kanji
japanki quiz some verb time unit --meaning                # quiz for meaning of vocabs; no model answer checks
japanki quiz some verb time unit --kanji --no-progress    # no progress checks
```

### Manage study progress

#### Example usage
```
japanki progress now   # show current levels
japanki progress up    # level up when you are familiar with current level vocabs
japanki progress down  # level down when you feel bad
japanki progress reset # reset trackings
```


## Why doing this?

While there are plenty of resources for learning Japanese, still none of them satisfy my very pedantic requirement:

- I do not want textbook or videos because the pace is too slow.
- I do not want to learn vocabs in alphabetical order which most learning app does.
- I do not want to learn it in multiple-choice question format as it can give an illusion of fully knowing the vocabs. I want to learn by filling in the blanks.
- I want to learn vocabs in a flash-card / SRS manner (Hence the name *JapAnki* !)
- I want to have customized control over the database of vocabs.
- I want to do it on a computer like a software engineer.
- I want to learn Rust along the way.

The app focuses solely in learning vocabs and *kanji*. Of course grammar and listening plays a huge role in language learning, but I have already found some other apps and articles that suits my style of learning.

##  Preparing the data

The list of Japanese N5 vocabularies is obtained from 9elt's project [jlpt-n5-word-list](https://github.com/9elt/jlpt-n5-word-list) with further processing with basic Python:

- Converting into .csv format
- Adding romaji for learning using [Cutlet](https://github.com/polm/cutlet)
- Manually deciding a category for grouping of vocabularies as I don't enjoy learning vocabs in alphabetical order.
- Add level tag to each vocabs by roughly binning vocabs in each categories into 10 levels in alphabetical order

The data processing is a one-time process and therefore excluded from the repository.
