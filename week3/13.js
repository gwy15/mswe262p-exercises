// ES6
let fs = require("fs");

let data_storage = {
    content: '',
    init(file) {
        this.content = fs.readFileSync(file).toString();
    },
    words() {
        let replaced = this.content.replace(/[\W_]+/g, ' ');
        return replaced
            .split(' ')
            .filter((s) => s.length > 1)
            .map((s) => s.toLowerCase())
    }
}

let stop_words_obj = {
    words: new Set(),
    init() {
        let content = fs.readFileSync("../stop_words.txt").toString();
        content.split(',').forEach((w) => {
            this.words.add(w);
        });
    },
    is_stop_words(word) {
        return this.words.has(word);
    }
}

let counter = {
    counter: {},
    count(word) {
        this.counter[word] = 1 + (this.counter[word] || 0)
    },
    top(limit) {
        let entries = [];
        for (let key of Object.keys(this.counter)) {
            entries.push([key, this.counter[key]]);
        }
        entries.sort((a, b) => {
            return b[1] - a[1];
        });
        if (entries.length > limit) {
            entries.length = limit;
        }
        return entries;
    }
}


data_storage.init(process.argv[2])
stop_words_obj.init();
for (let word of data_storage.words()) {
    if (!stop_words_obj.is_stop_words(word)) {
        counter.count(word);
    }
}
for (let [word, count] of counter.top(25)) {
    console.log(`${word} - ${count}`);
}
