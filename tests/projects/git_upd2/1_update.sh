#!/usr/bin/env bash
date +%s> dates.txt &&
git add dates.txt &&
git commit -m "update"
