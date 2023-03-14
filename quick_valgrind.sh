executable=`ls -t ./target/debug/deps/ | grep -E "^libasm_tester-[0123456789abcdef]+$" | head -n 1`
valgrind --leak-check=full --trace-children=yes --show-leak-kinds=all --suppressions=./valgrind_suppression_files/patch_rusty_and_cargo_test.txt target/debug/deps/${executable} $1
