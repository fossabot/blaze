#!/usr/bin/env python

import random
import re
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
    if not replace_terms: return text
    cache = re.sub
    for term in replace_terms:
        text = cache(term, replace_terms.get(term), text)
    return text

#
# brute force implementation
#

def py_str_replace(text='', replace_terms={}):
    if not replace_terms: return text
    for term in replace_terms:
        text = text.replace(term, replace_terms.get(term))
    return text
