PWD    = $(shell pwd)

SOURCE = $(PWD)/src
TESTS  = $(PWD)/tests

all:
	cd $(SOURCE) && \
     cargo build --verbose --all

test:
	cd $(TESTS) && \
	   pytest --benchmark-enable test_py_regex.py && \
	   pytest --benchmark-enable test_py_replace.py

clean:
	rm -rfv $(SOURCE)/target
	find $(SOURCE) \
	     -type f \
	     -iname "*.py[pc]" \
	     -exec rm -fv "{}" \;
