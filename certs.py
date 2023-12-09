import requests
import json
import locale
import re
import string
import tldextract
import base64
from dataclasses import dataclass
import datetime
import logging
import time

from collections import OrderedDict

from OpenSSL import crypto
from util import MerkleTreeHeader
from util import parse_ctl_entry
import tldextract

ctl_log = requests.get('https://www.gstatic.com/ct/log_list/v3/log_list.json').json()

urls = []
for operators in ctl_log['operators']:
    for log in operators['logs']:
        log_url = log['url']
        urls.append(log_url)

total_certs = 0
# for url in urls:
#     try: 
#         print(url)
#         log_info = requests.get(f'{url}/ct/v1/get-sth').json()
#         total_certs += log_info['tree_size']
#     except:
#         continue

# print(f"Total Certs: {total_certs}")

# for url in urls:
#     print(url)

with open('./tld-list-basic.json') as f:
    d = json.load(f)
    print(d)

tlds = {}
# for tld in d:
    # tlds[tld] = open(f'tld/{tld}.txt', 'w')
tlds['com'] = open(f'tld/com.txt', 'w')
    
url = urls[0]
url = "https://oak.ct.letsencrypt.org/2023/"
log_info = requests.get(f'{url}/ct/v1/get-sth').json()
total_certs += log_info['tree_size']
for i in range(0, total_certs, 1000):
    print(i)
    cert = requests.get(f'{url}/ct/v1/get-entries?start={i}&end={i+1000}').json()
    for entry in cert['entries']:
        cert_info = parse_ctl_entry(entry, None)
        for domain in cert_info['leaf_cert']['all_domains']:
            print(domain)
            tld = tldextract.extract(domain).suffix
            if tld not in tlds:
                tlds[tld] = open(f'tld/{tld}.txt', 'w')
            tlds[tld].write(domain + '\n')

