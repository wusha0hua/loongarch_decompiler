for name in `cat instructions`
do
	echo "mod ${name,,};"
done

for name in `cat instructions`
do
	echo "pub use ${name,,}::*;"
done
