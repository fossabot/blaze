#!/usr/bin/env python

import random
import re
import string

all = ['ASCII']

ASCII = string.ascii_letters

#
# data generation
#

def generate_random_sample(length=10,
                           blocksize=1,
                           delim=''):
    sample = ''.join([ random.choice(ASCII) for i in range(length) ])
    return str(delim).join([ sample[i:i+blocksize]
                             for i in range(0, length, blocksize)])

#
# testing
#

def py_str_count(text='', substring=''):
    return str(text).count(str(substring))

def py_regex_replace(text='', replace_terms={}):
    if not replace_terms: return text
    re_sub = re.sub # cache function
    for term in replace_terms:
        text = re_sub(term, replace_terms.get(term), text)
    return text

def py_str_replace(text='', pattern='', replacement=''):
    return text.replace(pattern, replacement)

def py_str_replacen(text='', replace_terms={}):
    if not replace_terms: return text
    for term in replace_terms:
        text = text.replace(term, replace_terms.get(term))
    return text

def py_str_lower(text=''):
    return str(text).lower()

def py_str_upper(text=''):
    return str(text).upper()
