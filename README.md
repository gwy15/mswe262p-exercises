# Run

## week1

```bash
# compile
cd ~/mswe-242p/week1 && rustc -o main main.rs

#  run
cd ~/mswe-242p/week1 && ./main ../pride-and-prejudice.txt
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
# compile
cd week2
rustc -o week2-1 four.rs
rustc -o week2-2 five.rs
rustc -o week2-3 six.rs

# run
cd week2
./week2-1 ../pride-and-prejudice.txt
./week2-2 ../pride-and-prejudice.txt
./week2-3 ../pride-and-prejudice.txt
```

## week3

You could compile & run either the easy way (RECOMMENDED):
```bash
cd week3
cargo r --release --bin 12-1 ../pride-and-prejudice.txt
```

Or the manual way:
```bash
# compile
cd week3
rustc -o 12-1 12-1.rs

# run
cd week3
./12-1 ../pride-and-prejudice.txt
```

