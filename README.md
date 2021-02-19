# Run

## week1

```bash
# enter directory
cd week1

# compile
rustc -o main main.rs

#  run
./main ../pride-and-prejudice.txt
```

## week2

You could compile & run either the easy way (RECOMMENDED):
```bash
cd week2
cargo r --release --bin week2-1 ../pride-and-prejudice.txt
cargo r --release --bin week2-2 ../pride-and-prejudice.txt
cargo r --release --bin week2-3 ../pride-and-prejudice.txt
```

Or the manual way:
```bash
# enter directory
cd week2

# compile
rustc -o week2-1 four.rs
rustc -o week2-2 five.rs
rustc -o week2-3 six.rs

# run
./week2-1 ../pride-and-prejudice.txt
./week2-2 ../pride-and-prejudice.txt
./week2-3 ../pride-and-prejudice.txt
```


## week3

You could compile & run either the easy way (RECOMMENDED):
```bash
cd week3
cargo r --release --bin 12-1 ../pride-and-prejudice.txt
node 13.js ../pride-and-prejudice.txt
cargo r --release --bin 16 ../pride-and-prejudice.txt
```

Or the manual way:
```bash
# enter directory
cd week3

# compile
rustc -o 12-1 12-1.rs
rustc -o 16 16.rs

# run
./12-1 ../pride-and-prejudice.txt
node 13.js ../pride-and-prejudice.txt
./16 ../pride-and-prejudice.txt
```


## week4
```bash
cd week4
node 9-1.js ../pride-and-prejudice.txt
cargo r --release --bin 10-1 ../pride-and-prejudice.txt
```


## week5
```bash
# compile
cd week5
make
# run ex 11
#!! you will be prompted to enter a class name， for example, WordFrequencyController. !!
java Eleven ../pride-and-prejudice.txt

# run ex 20.1
java -jar ./Framework.jar ./App1.jar ../pride-and-prejudice.txt # run with app1
java -jar ./Framework.jar ./App2.jar ../pride-and-prejudice.txt # or, run with app2

```

## week6
```bash
cd week6
# 26-1, might take a while to fetch sqlite lib from internet.
cargo r --release --features sqlite --bin 26-1 ../pride-and-prejudice.txt
# 28
cargo r --release --bin 28 ../pride-and-prejudice.txt
```

## week7
```bash
cd week7

cargo r --release --bin 21 ../pride-and-prejudice.txt
cargo r --release --bin 22 ../pride-and-prejudice.txt
cargo r --release --bin 25 ../pride-and-prejudice.txt
```

