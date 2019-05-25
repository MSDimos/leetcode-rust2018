#!/bin/bash

count=0;

for dir in `ls`
do
  if [[ -d $dir ]]
  then
    cd "./${dir}";

    if `cargo clippy >/dev/null 2>&1`; then
      echo "\"${dir}\" done"
      let count+=1;
    else
      echo $?;
      echo -e "\n++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++";
      echo "${dir} can't be processed by clippy, beacause of some errors";
      echo "please solve these errors, run commands below and fix it";
      echo;
      echo "    cd ${PWD}"
      echo "    cargo clippy"
      echo -e "----------------------------------------------------------------\n";
    fi

    # remove useless files
    rm -rf ./target
    cd ../
  fi
done;

echo -e "\n${count} projects have been done."
