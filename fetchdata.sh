#!/bin/bash

# Download JMdict
wget -O JMdict.gz "http://ftp.edrdg.org/pub/Nihongo/JMdict_b.gz"

# Uncompress
gunzip JMdict.gz

# Now JMdict (XML) is ready
echo "JMdict is ready: JMdict"
