#!/usr/bin/env python

from __init__ import *

try: import blaze
except ImportError: raise

import random

def test_str_replace_10(benchmark):
    sample = generate_random_sample()
    pattern = random.choice(ASCII)
    repl = random.choice(ASCII)
    benchmark(blaze.replace,
              sample,
              pattern,
              repl)

def test_str_replace_100(benchmark):
    sample = generate_random_sample(length=100)
    pattern = random.choice(ASCII)
    repl = random.choice(ASCII)
    benchmark(blaze.replace,
              sample,
              pattern,
              repl)

# 1,000
def test_str_replace_1000(benchmark):
    sample = generate_random_sample(length=1000)
    pattern = random.choice(ASCII)
    repl = random.choice(ASCII)
    benchmark(blaze.replace,
              sample,
              pattern,
              repl)

# 10,000
def test_str_replace_10000(benchmark):
    sample = generate_random_sample(length=10000)
    pattern = random.choice(ASCII)
    repl = random.choice(ASCII)
    benchmark(blaze.replace,
              sample,
              pattern,
              repl)

# 100,000
def test_str_replace_100000(benchmark):
    sample = generate_random_sample(length=100000)
    pattern = random.choice(ASCII)
    repl = random.choice(ASCII)
    benchmark(blaze.replace,
              sample,
              pattern,
              repl)

# 1,000,000
def test_str_replace_1000000(benchmark):
    sample = generate_random_sample(length=1000000)
    pattern = random.choice(ASCII)
    repl = random.choice(ASCII)
    benchmark(blaze.replace,
              sample,
              pattern,
              repl)
