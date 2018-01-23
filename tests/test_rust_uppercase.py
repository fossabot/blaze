#!/usr/bin/env python

from __init__ import *

try: import blaze
except ImportError: raise

def blaze_str_upper(text=''):
    return blaze.to_upper(text)

def test_upper_10(benchmark):
    sample = generate_random_sample()
    benchmark(blaze_str_upper, sample)

def test_upper_100(benchmark):
    sample = generate_random_sample(length=100)
    benchmark(blaze_str_upper, sample)

# 1,000
def test_upper_1000(benchmark):
    sample = generate_random_sample(length=1000)
    benchmark(blaze_str_upper, sample)

# 10,000
def test_upper_10000(benchmark):
    sample = generate_random_sample(length=10000)
    benchmark(blaze_str_upper, sample)

# 100,000
def test_upper_100000(benchmark):
    sample = generate_random_sample(length=100000)
    benchmark(blaze_str_upper, sample)

# 1,000,000
def test_upper_1000000(benchmark):
    sample = generate_random_sample(length=1000000)
    benchmark(blaze_str_upper, sample)
