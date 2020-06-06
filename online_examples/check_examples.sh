for entry in ./examples/*
do
	bare_file_name=${entry#"./examples/"}
	bare_file_name=${bare_file_name%".rs"}
	echo "checking ${bare_file_name}"
	cargo web check --example $bare_file_name --features stdweb
done