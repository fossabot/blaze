#!/usr/bin/env python

from __init__ import *

try: import blaze
except ImportError: raise

def blaze_str_lower(text=''):
    return blaze.to_lower(text)

def test_lower_10(benchmark):
    sample = generate_random_sample()
    benchmark(blaze_str_lower, sample)

def test_lower_100(benchmark):
    sample = generate_random_sample(length=100)
    benchmark(blaze_str_lower, sample)

# 1,000
def test_lower_1000(benchmark):
    sample = generate_random_sample(length=1000)
    benchmark(blaze_str_lower, sample)

# 10,000
def test_lower_10000(benchmark):
    sample = generate_random_sample(length=10000)
    benchmark(blaze_str_lower, sample)

# 100,000
def test_lower_100000(benchmark):
    sample = generate_random_sample(length=100000)
    benchmark(blaze_str_lower, sample)

# 1,000,000
def test_lower_1000000(benchmark):
    sample = generate_random_sample(length=1000000)
    benchmark(blaze_str_lower, sample)
