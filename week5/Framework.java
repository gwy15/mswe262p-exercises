// Do exercise 20.1 as specified in the book. If you are doing this in Java or C#,
// you should produce three jar (dll in C#) files: the framework package 
// (framework.jar or Framework.dll), a package with one implementation of the words and
// frequencies functions (app1.jar or App1.dll), and another package with another
// implementation of the words and frequencies functions (app2.jar or App2.dll).
// One should be able to run the framework plus one of the apps from anywhere,
// as long as they have those packages and the configuration file.

import java.io.File;
import java.lang.reflect.*;
import java.net.URL;
import java.net.URLClassLoader;
import java.util.ArrayList;

public class Framework {
    public static void main(String[] args) throws Exception {
        if (args.length < 2) {
            System.err.println("Usage: ./Framework <path-to-app-jar> <path-to-txt-file>");
            return;
        }
        String appPath = args[0];
        String filePath = args[1];

        // refer:
        // https://stackoverflow.com/questions/60764/how-to-load-jar-files-dynamically-at-runtime
        File appFile = new File(appPath);
        URLClassLoader child = new URLClassLoader(new URL[] { appFile.toURI().toURL() },
                Framework.class.getClassLoader());
        Class App = Class.forName("App", true, child);
        Object app = App.getConstructor(String.class).newInstance(filePath);
        ArrayList<String> words = (ArrayList<String>) App.getMethod("getWords").invoke(app);

        Method printF = App.getMethod("print", (ArrayList.class));
        printF.invoke(null, words);
    }
}
