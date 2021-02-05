import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Set;

public class App {
    String filename;

    public App(String filename) {
        this.filename = filename;
    }

    public ArrayList<String> getWords() throws Exception {
        HashSet<String> stopWords = new HashSet();
        try (BufferedReader reader = new BufferedReader(new FileReader("../stop_words.txt"))) {
            reader.lines().forEach(line -> {
                for (String w : line.split(",")) {
                    stopWords.add(w);
                }
            });
        }
        try (BufferedReader reader = new BufferedReader(new FileReader(this.filename))) {
            ArrayList<String> words = new ArrayList<>();
            reader.lines().forEach(line -> {
                for (String w : line.split("[\\W_]+")) {
                    w = w.toLowerCase();
                    if (w.length() > 1 && !stopWords.contains(w)) {
                        words.add(w);
                    }
                }
            });
            return words;
        }
    }

    public static void print(ArrayList<String> words) {
        HashMap<String, Integer> counter = new HashMap<>();
        for (String word : words) {
            Integer _v = counter.get(word);
            int count = _v == null ? 0 : _v;
            counter.put(word, count + 1);
        }
        ArrayList<String> keys = new ArrayList(counter.keySet());
        keys.sort((k1, k2) -> {
            return counter.get(k2) - counter.get(k1);
        });
        for (int i = 0; i < Math.min(25, keys.size()); i++) {
            System.out.println(keys.get(i) + " - " + counter.get(keys.get(i)));
        }
    }

    public static void main(String[] args) throws Exception {
        App app = new App("../pride-and-prejudice.txt");
        ArrayList<String> words = app.getWords();
        System.out.println("words: " + words.size());
        App.print(words);
    }
}
