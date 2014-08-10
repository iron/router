SRCDIR = src
BUILDDIR = target
DEPSDIR = target/deps
NATIVEDIR = target/native
EXAMPLEDIR = examples

all: build doc examples

.PHONY: clean build

build :
	cargo build

clean :
	cargo clean

test :
	cargo test

doc :
	cargo doc

examples : $(BUILDDIR) $(DEPSDIR) $(NATIVEDIR)
	rustc -g $(EXAMPLEDIR)/simple.rs -L $(BUILDDIR) -L $(DEPSDIR) -L $(NATIVEDIR) --out-dir $(EXAMPLEDIR)
