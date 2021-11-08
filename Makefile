testdir=./test_dir/plugins

.PHONY: all install install_test test run format clippy

all: src/*
	cargo build --release

all_debug: src/*
	cargo build

.ONESHELL:
install: all
	if [[ ! -z $${WEECHAT_HOME} ]]; then
	  installdir=$${WEECHAT_HOME}/plugins
	elif [[ ! -z $${XDG_DATA_HOME} ]]; then
	  installdir=$${XDG_DATA_HOME}/weechat/plugins
	else
	  installdir=$${HOME}/.weechat/plugins
	fi
	mkdir -p $${installdir}
	cp target/release/libweecord.* $${installdir}

install_test: all_debug
	mkdir -p $(testdir)
	cp target/debug/libweecord.* $(testdir)

run: install
	weechat -a

test: install_test
	weechat -d $(testdir)

format:
	cargo fmt
