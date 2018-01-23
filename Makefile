PROJECT = $(shell pwd)

SOURCE = $(PROJECT)/src
TESTS  = $(PROJECT)/tests

all: # Python 2.7
	cd $(SOURCE) && \
	   cargo build --verbose --all && \
	   mv $(SOURCE)/target/debug/libblaze.so \
	      $(PROJECT)/blaze.so

release: # Python 2.7
	cd $(SOURCE) && \
	   cargo build --release --verbose --all && \
	   mv $(SOURCE)/target/release/libblaze.so \
	      $(PROJECT)/blaze.so

test:
	cd $(TESTS) && \
	   pytest --benchmark-enable test_py_regex.py && \
	   pytest --benchmark-enable test_py_replace.py && \
	   pytest --benchmark-enable test_py_lowercase.py && \
	   pytest --benchmark-enable test_py_uppercase.py

clean:
	rm -rfv $(SOURCE)/target
	rm -fv $(PROJECT)/blaze.so
	find $(SOURCE) \
	     -type f \
	     -iname "*.py[pc]" \
	     -exec rm -fv "{}" \;
