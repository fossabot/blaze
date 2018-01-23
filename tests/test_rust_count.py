#!/usr/bin/env python

from __init__ import *

try: import blaze
except ImportError: raise

import random

def blaze_str_count(text='', pattern=''):
    return blaze.to_upper(text)

def test_count_10(benchmark):
    sample = generate_random_sample()
    choice = random.choice(ASCII)
    benchmark(blaze_str_count, sample, choice)

def test_count_100(benchmark):
    sample = generate_random_sample(length=100)
    choice = random.choice(ASCII)
    benchmark(blaze_str_count, sample, choice)

# 1,000
def test_count_1000(benchmark):
    sample = generate_random_sample(length=1000)
    choice = random.choice(ASCII)
    benchmark(blaze_str_count, sample, choice)

# 10,000
def test_count_10000(benchmark):
    sample = generate_random_sample(length=10000)
    choice = random.choice(ASCII)
    benchmark(blaze_str_count, sample, choice)

# 100,000
def test_count_100000(benchmark):
    sample = generate_random_sample(length=100000)
    choice = random.choice(ASCII)
    benchmark(blaze_str_count, sample, choice)

# 1,000,000
def test_count_1000000(benchmark):
    sample = generate_random_sample(length=1000000)
    choice = random.choice(ASCII)
    benchmark(blaze_str_count, sample, choice)
