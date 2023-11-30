alias s := scaffold
alias r := run

scaffold day:
	cargo new day{{day}}
	cargo add -p day{{day}} --path common/
	cat resources/template.rs > day{{day}}/src/main.rs
	aoc d -d {{day}} -I -i day{{day}}/input.in

run day:
	cargo run -p day{{day}}
