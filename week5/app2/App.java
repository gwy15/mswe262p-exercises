import java.io.BufferedReader;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Scanner;
import java.util.Set;

public class App {
    String filename;

    public App(String filename) {
        this.filename = filename;
    }

    public ArrayList<String> getWords() throws Exception {
        HashSet<String> stopWords = new HashSet();
        try (Scanner f = new Scanner(new FileReader("../stop_words.txt"))) {
            f.useDelimiter(",");
            while (f.hasNext()) {
                stopWords.add(f.next());
            }
        }
        try (Scanner f = new Scanner(new FileReader(this.filename))) {
            f.useDelimiter("[\\W_]+");
            ArrayList<String> words = new ArrayList<>();
            while (f.hasNext()) {
                String w = f.next().toLowerCase();
                if (w.length() > 1 && !stopWords.contains(w)) {
                    words.add(w);
                }

            }
            return words;
        }
    }

    public static void print(ArrayList<String> words) {
        HashMap<String, Integer> counter = new HashMap<>();
        for (String word : words) {
            Integer count = counter.get(word);
            if (count == null) {
                counter.put(word, 1);
            } else {
                counter.put(word, count + 1);
            }
        }
        ArrayList<String> keys = new ArrayList(counter.keySet());
        keys.sort((k1, k2) -> {
            return counter.get(k2) - counter.get(k1);
        });
        for (int i = 0; i < keys.size(); i++) {
            System.out.println(keys.get(i) + " - " + counter.get(keys.get(i)));
            if (i >= 24) {
                break;
            }
        }
    }

    public static void main(String[] args) throws Exception {
        App app = new App("../pride-and-prejudice.txt");
        ArrayList<String> words = app.getWords();
        System.out.println("words: " + words.size());
        App.print(words);
    }
}
