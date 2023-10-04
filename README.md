<div style="text-align: center">
    <h1>JapAnki👹</h1>
    <h5>Learn Japanese N5 vocabularies and do quizzes on CLI!</h5>
</div>

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

The data processing is a one-time process and therefore excluded from the repository.
