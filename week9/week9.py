from os import W_OK
import sys
import string
import numpy as np


MAPPING = str.maketrans({
    'A': '4',
    'B': '8',
    'C': '(',
    'I': '1',
    'N': 'И',
    'O': '0',
    'R': 'Я',
    'S': '5',
    'T': '7',
    'Z': '2'
})


def get_characters():
    characters = np.array(
        [' ']+list(open(sys.argv[1]).read())+[' '])
    return characters


def normalize(characters):
    characters[~np.char.isalpha(characters)] = ' '
    characters = np.char.upper(characters)
    return characters


def characters_to_words(characters):
    space_indexes = np.where(characters == ' ')[0]
    words_ranges = np.stack((space_indexes[:-1], space_indexes[1:]), axis=-1)
    # words.length >= 2 => >2
    words_ranges = words_ranges[np.where(
        words_ranges[:, 1] - words_ranges[:, 0] > 2)]

    def f(r):
        return ''.join(characters[r[0]+1:r[1]])
    # words = np.apply_along_axis(f, 1, words_ranges)
    words = np.array([f(r) for r in words_ranges])

    return words


def filter_stopwords(words):
    stop_words = np.char.upper(
        np.array(list(set(open('../stop_words.txt').read().split(',')))))
    words = words[~np.isin(words, stop_words)]
    return words


def map_leet(words):
    return np.char.translate(words, MAPPING)


def count_2gram(words):
    two_grams = np.stack((words[:-1], words[1:]), axis=-1)
    pairs, counts = np.unique(two_grams, axis=0, return_counts=True)
    pairs_counts = sorted(zip(pairs, counts), key=lambda t: t[1], reverse=True)
    return pairs_counts


characters = get_characters()
characters = normalize(characters)
words = characters_to_words(characters)
words = filter_stopwords(words)
words = map_leet(words)
pair_counts = count_2gram(words)

for pair, counts in pair_counts[:5]:
    print(pair, '-', counts)
