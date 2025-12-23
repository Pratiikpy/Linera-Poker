#!/bin/bash
# Remove Claude watermark lines
sed -e '/🤖 Generated with/d' \
    -e '/Co-Authored-By: Claude/d' \
    -e '/^$/N;/^\n$/d'
