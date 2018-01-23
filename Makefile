PWD    = $(shell pwd)

SOURCE = $(PWD)/src
TESTS  = $(PWD)/tests

test:
	pytest --benchmark-enable $(TESTS)/test_py_regex.py
	pytest --benchmark-enable $(TESTS)/test_py_replace.py
