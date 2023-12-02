#!/usr/bin/env bash

# this is a script downloads puzzle input for a particular day 
# Pass in the day as the first argument to the script
#
# to set this up, you need to have filled a file called session_cookie.txt with the cookie
# named "session" which can be obtained from the browser developer console after you
# have logged into advent of code.
# The format is in netscape cookie format (for an example, do wget --save-cookies=example.txt https://google.com)
#

YEAR=${YEAR:-$(date +%Y)}

if [[ $# != 1 ]]
then
  echo Usage: $0 \<DAY OF ADVENT\>
  exit 1
fi

BASE_PATH="$(dirname $(realpath $0))"

SAVE_FILE=input$1.txt

URL="https://adventofcode.com/$YEAR/day/$1/input"

curl -b "$BASE_PATH"/session_cookie.txt "$URL" -o "$SAVE_FILE"
