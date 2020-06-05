
CC = gcc
LDFLAGS := -lpthread -ldl

main: main.o rustc/target/debug/librustc.a
	gcc -o $@ $^ $(LDFLAGS)

rustc/target/debug/librustc.a: rustc/src/lib.rs rustc/Cargo.toml
	cd rustc; cargo build

main.o: main.c
	$(CC) -o $@ -c $<

clean:
	rm rustc/target/debug/librustc.a
	rm *.o
	rm main
