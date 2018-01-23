#!/usr/bin/env python

import random
import string

all = ['ASCII']

ASCII = string.ascii_letters

#
# sample data generator
#

def generate_random_sample(length=10,
                           blocksize=1,
                           delim=''):
    sample = ''.join([ random.choice(ASCII) for i in range(length) ])
    return str(delim).join([ sample[i:i+blocksize]
                             for i in range(0, length, blocksize)])

#
# regex implementation
#

def py_regex_replace(text='', replace_terms={}):
    terms = text.split()
    return

#
# brute force implementation
#

def py_str_replace(text='', replace_terms={}):
    terms = text.split()
    return
