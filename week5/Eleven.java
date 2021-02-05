// In the run method of WordFrequencyController (or equivalent in your code,
// style 11), invoke the methods of the DataStorageManager, StopWordManager, and
// WordFrequencyCounter objects using reflection.
//
// In your main function (or main block), prompt the user for a class name of
// one of your application classes (e.g. DataStorageManager) and call a function
// that prints out all of that class's fields (their names and types), all of
// its method names, and all of its superclasses and implemented interfaces --
// this should be done using reflection, and not hardcoded with conditionals
// (harcoding with conditionals would be you doing if the user chose
// DataStorageManager let me printout the fields I know it has, etc.). The
// printout function should take a class name as string and should work in a
// generic manner for any class that exists in your application.
//

import java.io.*;
import java.util.*;

import java.lang.reflect.*;

// to demonstrate that the superclasses can be printed
class Parent {
}

class WordFrequencyController extends Parent {
    private DataStorageManager storageManager;
    private StopWordManager stopWordManager;
    private WordFrequencyManager wordFreqManager;

    public WordFrequencyController(String pathToFile) throws IOException {
        this.storageManager = new DataStorageManager(pathToFile);
        this.stopWordManager = new StopWordManager();
        this.wordFreqManager = new WordFrequencyManager();
    }

    public void run() throws Exception {
        Method getWords = this.storageManager.getClass().getMethod("getWords");
        Method isStopWord = this.stopWordManager.getClass().getMethod("isStopWord", String.class);
        Method incrementCount = this.wordFreqManager.getClass().getMethod("incrementCount", String.class);

        for (String word : (List<String>) getWords.invoke(storageManager)) {
            if (!(boolean) isStopWord.invoke(stopWordManager, word)) {
                incrementCount.invoke(wordFreqManager, word);
            }
        }

        Method sorted = this.wordFreqManager.getClass().getMethod("sorted");
        Method getWord = WordFrequencyPair.class.getMethod("getWord");
        Method getFrequency = WordFrequencyPair.class.getMethod("getFrequency");

        int numWordsPrinted = 0;
        for (WordFrequencyPair pair : (List<WordFrequencyPair>) sorted.invoke(wordFreqManager)) {
            System.out.println((String) getWord.invoke(pair) + " - " + (Integer) getFrequency.invoke(pair));

            numWordsPrinted++;
            if (numWordsPrinted >= 25) {
                break;
            }
        }
    }
}

/** Models the contents of the file. */
class DataStorageManager {
    private List<String> words;

    public DataStorageManager(String pathToFile) throws IOException {
        this.words = new ArrayList<String>();

        Scanner f = new Scanner(new File(pathToFile), "UTF-8");
        try {
            f.useDelimiter("[\\W_]+");
            while (f.hasNext()) {
                this.words.add(f.next().toLowerCase());
            }
        } finally {
            f.close();
        }
    }

    public List<String> getWords() {
        return this.words;
    }
}

/** Models the stop word filter. */
class StopWordManager {
    private Set<String> stopWords;

    public StopWordManager() throws IOException {
        this.stopWords = new HashSet<String>();

        Scanner f = new Scanner(new File("../stop_words.txt"), "UTF-8");
        try {
            f.useDelimiter(",");
            while (f.hasNext()) {
                this.stopWords.add(f.next());
            }
        } finally {
            f.close();
        }

        // Add single-letter words
        for (char c = 'a'; c <= 'z'; c++) {
            this.stopWords.add("" + c);
        }
    }

    public boolean isStopWord(String word) {
        return this.stopWords.contains(word);
    }
}

/** Keeps the word frequency data. */
class WordFrequencyManager {
    private Map<String, MutableInteger> wordFreqs;

    public WordFrequencyManager() {
        this.wordFreqs = new HashMap<String, MutableInteger>();
    }

    public void incrementCount(String word) {
        MutableInteger count = this.wordFreqs.get(word);
        if (count == null) {
            this.wordFreqs.put(word, new MutableInteger(1));
        } else {
            count.setValue(count.getValue() + 1);
        }
    }

    public List<WordFrequencyPair> sorted() {
        List<WordFrequencyPair> pairs = new ArrayList<WordFrequencyPair>();
        for (Map.Entry<String, MutableInteger> entry : wordFreqs.entrySet()) {
            pairs.add(new WordFrequencyPair(entry.getKey(), entry.getValue().getValue()));
        }
        Collections.sort(pairs);
        Collections.reverse(pairs);
        return pairs;
    }

}

class MutableInteger {
    private int value;

    public MutableInteger(int value) {
        this.value = value;
    }

    public int getValue() {
        return value;
    }

    public void setValue(int value) {
        this.value = value;
    }
}

class WordFrequencyPair implements Comparable<WordFrequencyPair> {
    private String word;
    private int frequency;

    public WordFrequencyPair(String word, int frequency) {
        this.word = word;
        this.frequency = frequency;
    }

    public String getWord() {
        return word;
    }

    public int getFrequency() {
        return frequency;
    }

    public int compareTo(WordFrequencyPair other) {
        return this.frequency - other.frequency;
    }
}

class Eleven {
    public static void printout(String className) {
        try {
            Class cls = Class.forName(className);

            for (Field field : cls.getDeclaredFields()) {
                System.out.println("[1] class " + className + " field : " + field.getName() + ": "
                        + field.getType().getCanonicalName());
            }

            for (Method method : cls.getDeclaredMethods()) {
                System.out.println("[2] class " + className + " method: " + method.toString());
            }

            Class _cls = cls.getSuperclass();
            while (_cls != null) {
                System.out.println("[3] class " + className + " superclass: " + _cls.getCanonicalName());
                _cls = _cls.getSuperclass();
            }

            for (Class i : cls.getInterfaces()) {
                System.out.println("[4] class " + className + " interface: " + i.getCanonicalName());
            }
        } catch (ClassNotFoundException e) {
            System.err.println("Class " + className + " not found.");
        }
    }

    public static void main(String[] args) throws Exception {
        // run reflection
        System.out.print("class name:> ");
        BufferedReader reader = new BufferedReader(new InputStreamReader(System.in));
        String input = reader.readLine();
        printout(input);

        // run the word counter
        String file = args[0];
        new WordFrequencyController(file).run();
    }
}
