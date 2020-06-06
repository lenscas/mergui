for entry in ./examples/*
do
	bare_file_name=${entry#"./examples/"}
	bare_file_name=${bare_file_name%".rs"}
	echo "Compiling ${bare_file_name}"
	cargo web build --release --example $bare_file_name --features stdweb
	sed "s/REPLACE_ME/${bare_file_name}/" ./online_examples/example.html > "./preps/${bare_file_name}.html"
done