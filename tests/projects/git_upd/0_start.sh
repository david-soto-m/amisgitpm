#!/usr/bin/env bash
git init . &&
date +%s > dates.txt &&
git add dates.txt &&
git commit -m "start"
