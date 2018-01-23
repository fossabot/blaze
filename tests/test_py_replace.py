#!/usr/bin/env python

from __init__ import *

replace_terms = dict(zip(string.ascii_letters,
                         string.ascii_letters.encode('rot-13')))

def test_str_replace_10(benchmark):
    sample = generate_random_sample()
    benchmark(py_str_replace,
              sample,
              replace_terms)

def test_str_replace_100(benchmark):
    sample = generate_random_sample(length=100)
    benchmark(py_str_replace,
              sample,
              replace_terms)

# 1,000
def test_str_replace_1000(benchmark):
    sample = generate_random_sample(length=1000)
    benchmark(py_str_replace,
              sample,
              replace_terms)

# 10,000
def test_str_replace_10000(benchmark):
    sample = generate_random_sample(length=10000)
    benchmark(py_str_replace,
              sample,
              replace_terms)

# 100,000
def test_str_replace_100000(benchmark):
    sample = generate_random_sample(length=100000)
    benchmark(py_str_replace,
              sample,
              replace_terms)

# 1,000,000
def test_str_replace_1000000(benchmark):
    sample = generate_random_sample(length=1000000)
    benchmark(py_str_replace,
              sample,
              replace_terms)
