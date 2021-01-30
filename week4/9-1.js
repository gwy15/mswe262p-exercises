//! Style #9
//! ==============================
//! Variation of the candy factory style, with the following additional constraints:
//! - Each function takes an additional parameter, usually the last, which is another function
//! - That function parameter is applied at the end of the current function
//! - That function parameter is given as input what would be the output of the current function
//! - Larger problem is solved as a pipeline of functions, but where the next function to be
//!   applied is given as parameter to the current function
//! Possible names:
//! - Kick your teammate forward!
//! - Continuation-passing style
//! - Crochet loop

let fs = require("fs");

function read_file(path_to_file, func) {
    let content = fs.readFileSync(path_to_file).toString();
    func(content, normalize)
}

function filter_chars(str_data, func) {
    func(str_data.replace(/[\W_]+/g, ' '), scan)
}

function normalize(str_data, func) {
    func(str_data.toLowerCase(), remove_stop_words)
}

function scan(str_data, func) {
    func(str_data.split(' '), frequencies)
}

function remove_stop_words(word_list, func) {
    let stop_words = fs.readFileSync("../stop_words.txt").toString().split(',');
    let stop_words_set = new Set(stop_words);
    word_list = word_list.filter((w) => {
        return w.length > 1 && !stop_words_set.has(w);
    })
    func(word_list, sort)
}

function frequencies(word_list, func) {
    let counter = {}
    for (let w of word_list) {
        counter[w] = (counter[w] || 0) + 1
    }
    func(counter, print)
}

function sort(word_freqs, func) {
    let items = Object.keys(word_freqs).map((w) => [w, word_freqs[w]])
        .sort((tup1, tup2) => tup2[1] - tup1[1]);
    func(items, no_op)
}


function print(word_freqs, func) {
    if (word_freqs.length > 25) {
        word_freqs.length = 25;
    }
    for (let [w, c] of word_freqs) {
        console.log(`${w} - ${c}`)
    }
    func(null)
}

function no_op(func) { }

read_file(process.argv[2], filter_chars)
