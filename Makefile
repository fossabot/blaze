PROJECT = $(shell pwd)

SOURCE = $(PROJECT)/src
TESTS  = $(PROJECT)/tests

all:
	cd $(SOURCE) && \
	   cargo build --verbose --all && \
	   mv $(SOURCE)/target/debug/libblazelib.so \
	      $(PROJECT)/blazelib.so

release:
	cd $(SOURCE) && \
	   cargo build --release --verbose --all && \
	   mv $(SOURCE)/target/release/libblazelib.so \
	      $(PROJECT)/blazelib.so

test:
	cd $(TESTS) && \
	   pytest --benchmark-enable test_py_regex.py && \
	   pytest --benchmark-enable test_py_replace.py

clean:
	rm -rfv $(SOURCE)/target
	rm -fv $(PROJECT)/blazelib.so
	find $(SOURCE) \
	     -type f \
	     -iname "*.py[pc]" \
	     -exec rm -fv "{}" \;
