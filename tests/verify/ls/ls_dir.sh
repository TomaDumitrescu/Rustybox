#!/bin/bash
# 10

outputfile=$1
testfile=$2

rm -rf lstest

mkdir lstest
touch lstest/modern
touch lstest/family
mkdir lstest/gloria
touch lstest/.hidden

ls lstest | grep -v total > output/ls_out

rustybox ls `pwd`/lstest &> $outputfile
scriptresult=$?

node verify/ls/ls.js output/ls_out $outputfile > $testfile 2>> $outputfile
testresult=$?

rm -rf lstest
rm -f output/ls_out

if [ $testresult == 0 ]
then
    if [ $scriptresult != 0 ]
    then
        echo "Correct ls does not return 0 exit code." > $testfile
        exit -1 
    fi
fi

exit $testresult