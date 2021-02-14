all:
	javac *.java
	cd app1 && javac *.java && jar -cf App.jar App.class && mv App.jar ../App1.jar
	cd app2 && javac *.java && jar -cf App.jar App.class && mv App.jar ../App2.jar
	jar -cfm Framework.jar manifest.mf Framework.class

clean:
	rm -f *.class *.jar
	cd app1 && rm -f *.class *.jar
	cd app2 && rm -f *.class *.jar

.PHONY: all clean

