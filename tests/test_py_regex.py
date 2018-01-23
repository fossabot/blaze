#!/usr/bin/env python

from __init__ import *

import re
import sys

replace_terms = dict(zip(string.ascii_letters,
                         string.ascii_letters.encode('rot-13')))

def test_regex_replace_10(benchmark):
    sample = generate_random_sample()
    sys.stdout.write(benchmark(py_regex_replace,
                               sample,
                               replace_terms))

def test_regex_replace_100(benchmark):
    sample = generate_random_sample(length=100)
    sys.stdout.write(benchmark(py_regex_replace,
                               sample,
                               replace_terms))

# 1,000
def test_regex_replace_1000(benchmark):
    sample = generate_random_sample(length=1000)
    sys.stdout.write(benchmark(py_regex_replace,
                               sample,
                               replace_terms))

# 10,000
def test_regex_replace_10000(benchmark):
    sample = generate_random_sample(length=10000)
    sys.stdout.write(benchmark(py_regex_replace,
                               sample,
                               replace_terms))

# 100,000
def test_regex_replace_100000(benchmark):
    sample = generate_random_sample(length=100000)
    sys.stdout.write(benchmark(py_regex_replace,
                               sample,
                               replace_terms))

# 1,000,000
def test_regex_replace_1000000(benchmark):
    sample = generate_random_sample(length=1000000)
    sys.stdout.write(benchmark(py_regex_replace,
                               sample,
                               replace_terms))

# 10,000,000
def test_regex_replace_10000000(benchmark):
    sample = generate_random_sample(length=10000000)
    sys.stdout.write(benchmark(py_regex_replace,
                               sample,
                               replace_terms))
