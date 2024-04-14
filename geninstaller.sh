#!/bin/sh
if [ $# -ne 4 ]; then
echo Argument count wrong!
exit 1
fi
real_in=$(realpath $1)
real_out=$(realpath $2)

extract_dir=temp/a

mkdir -p $extract_dir
extract_dir=$(realpath $extract_dir)
cd $extract_dir
jar -x --file $real_in install_profile.json version.json
cd -
cp -r pillow-data temp/b

version_id=`cargo run version-json $extract_dir/version.json temp/b/version.json $3 $4`
cargo run install-profile $extract_dir/install_profile.json temp/b/install_profile.json $version_id

cp $real_in $real_out
jar -u --file $real_out -C temp/b .
