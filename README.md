<div style="text-align: center">
    <h1>JapAnki👹</h1>
    <h5>Learn Japanese N5 vocabularies and do quizzes on CLI!</h5>
</div>

- [Installation](#installation)
  - [Build from source](#build-from-source)
- [Introduction](#introduction)
- [Features and usage](#features-and-usage)
  - [List catories](#list-catories)
    - [Example usage](#example-usage)
  - [Show Japanese vocabularies](#show-japanese-vocabularies)
    - [Example](#example)
  - [Quiz yourself](#quiz-yourself)
    - [Example usage](#example-usage-1)
  - [Manage study progress](#manage-study-progress)
    - [Example usage](#example-usage-2)
  - [Display all vocabs](#display-all-vocabs)
- [Why doing this?](#why-doing-this)
- [Preparing the data](#preparing-the-data)


## Installation

### Build from source

- Clone the repository.
- Have latest Rust installed, build with `cargo build` and obtain the binary.
- You will also need to have the vocab csv file and progress .yaml file at `$HOME/.japanki`. Copy `./.japanki` directory in this repo to your home directory.

- `cd` to the binary directory, usually at `./target/debug/`, and run `./japanki` with arguments (see below for usage).
- Optionally if you wish to have `japanki` installed on system startup, copy the binary to `/usr/local/bin`.

This above steps are outlined by a very simple install script. Run `./install.sh`.

## Introduction

- This simple CLI let's you study and memorize japanese N5 vocabs in terminal, like this:

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
...

```

- Or you can do quiz:

```
>> japanki quiz all --kanji

====== Unit 0 | Level 1 =====
Hiragana: ??? | ～駅 | Romaji: ???
Meaning: ... station

> Romaji is: eki
✅ Correct! It is ～えき | ～駅 | ~ eki

> Next question ▷

====== Nature 78 | Level 1 =====
Hiragana: ??? | 雨 | Romaji: ???
Meaning: rain

> Romaji is: ami
❌ Oops! It should be あめ | 雨 | Ame

> ✏️ Correction: ame
```

- You can manage your progress, level up when you are familiar with current sets of vocabs:

```
>> japanki progress now

=== Current progress ===
Unit          Level  1 / 10
Atomic        Level  3 / 10
Time          Level  2 / 10
People        Level  4 / 10
Places        Level  5 / 10
Verb          Level  1 / 10
```


## Features and usage

### List catories

#### Example usage

```bash
japanki list   # list all available categories

# Available categories:
#  - Unit
#  - Atomic
#  - Time
#  - People
#  - Places
#  - Verb
#  - Adjadv
#  - Color
#  - Direction
#  - Nature
#  - Food
#  - Body
#  - Home
#  - Intangible
#  - Activity
#  - Wearables
#  - Manmade
#  - Stationery
#  - Transport
#  - Sentence
```

### Show Japanese vocabularies

#### Example

```bash
# show vocabs in all categories
japanki show all

# show within the verb/time/unit category
japanki show some verb time unit

# show vocabs that involves kanji
japanki show some verb time unit --kanji

# no progress filtering when showing vocabs
japanki show some verb time unit --kanji --no-progress
```

### Quiz yourself

#### Example usage

```bash
# quiz with vocabs in all categories
japanki quiz all

# quiz within the verb/time/unit category
japanki quiz some verb time unit

# quiz vocabs that involves kanji
japanki quiz some verb time unit --kanji

# quiz for meaning of vocabs; no model answer checks
japanki quiz some verb time unit --meaning

# no progress filtering when quiz
japanki quiz some verb time unit --kanji --no-progress
```

### Manage study progress

#### Example usage

```bash
# show current levels
japanki progress now

# level up when you are familiar with current level vocabs
japanki progress up

# level down when you feel bad
japanki progress down

# reset trackings
japanki progress reset
```

### Display all vocabs

```bash
# display all
japanki display all

# display selected categories
japanki display some verb time unit
```

## Why doing this?

While there are plenty of resources for learning Japanese, still, none of them satisfy my very pedantic needs:

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
