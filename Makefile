build:
	mkdir -p lib && rustc src/lib.rs --lib -o lib/fsm

test:
	mkdir -p bin && rustc src/lib.rs -o bin/fsmtest --test
	./bin/fsmtest

docs:
	mkdir -p docs && rustdoc src/lib.rs -o docs/

docstest:
	mkdir -p docs && rustdoc src/lib.rs -o docs/ --test

.PHONY: build docs test