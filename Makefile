rwildcard = $(strip $(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d)))

# recursively find all the .proto
PROTOS := $(strip $(call rwildcard,src,*.proto))
filter := src/blockchain_block.proto src/blockchain_block_v1.proto src/service/gateway.proto src/service/router.proto
PROTOS := $(filter-out $(filter),$(PROTOS))

# create C_SRCS which will be our build targets
C_SRCS := $(patsubst src/%.proto,build/%.c, $(PROTOS))

# create list of all directories needed for build output
DIRS := build $(patsubst src%,build%,$(dir $(C_SRCS)))

.PHONY: build clean dirs

PROTOC := protoc -I$(HOME)/nanopb/generator/proto --plugin=protoc-gen-nanopb=$(HOME)/nanopb/generator/protoc-gen-nanopb
OUT := --nanopb_out

build: $(C_SRCS)

dirs:
	mkdir -p $(DIRS)

clean:
	rm -rf build

build/%.c: src/%.proto dirs
	$(PROTOC) $(OUT)=$(@D) $(<F) --proto_path $(<D)
