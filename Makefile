PWD    = $(shell pwd)

SOURCE = $(PWD)/src
TESTS  = $(PWD)/tests

test:
	cd $(TESTS)
	pytest test_py_regex.py
	pytest test_py_replace.py
