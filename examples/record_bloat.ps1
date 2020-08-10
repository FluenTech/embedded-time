cargo bloat --bin isolated --target thumbv7em-none-eabihf --release --wide -n 100 | Out-File isolated-bloat.txt -Encoding UTF8
