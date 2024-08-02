#!/bin/sh
set -e
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
jar -x --file $real_in install_profile.json version.json data/unix_args.txt data/win_args.txt
cd -
cp -r pillow-data temp/b
mkdir temp/b/data

version_id=`cargo run --release version-json $extract_dir/version.json temp/b/version.json $3 $4`
cargo run --release install-profile $extract_dir/install_profile.json temp/b/install_profile.json $3 $4 $version_id
cargo run --release jvm-args $extract_dir/data/unix_args.txt temp/b/data/unix_args.txt $3 $4
cargo run --release jvm-args $extract_dir/data/win_args.txt temp/b/data/win_args.txt $3 $4 --windows

cp $real_in $real_out
zip -d $real_out data/client.lzma data/server.lzma
jar -u --file $real_out -C temp/b .
