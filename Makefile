rwildcard = $(strip $(foreach d,$(wildcard $(1:=/*)),$(call rwildcard,$d,$2) $(filter $(subst *,%,$2),$d)))
PROTOS := $(strip $(call rwildcard,src,*.proto))
C_SRCS := $(patsubst src/%.proto,build/%.c, $(PROTOS))
C_HEADERS := $(patsubst src/%.proto,build/%.h, $(PROTOS))

DIRS := build $(patsubst src%,build%,$(dir $(C_SRCS)))

$(info $(DIRS))

.PHONY: build clean dirs

build: $(C_SRCS)

dirs:
	mkdir -p $(DIRS)

clean:
	rm -rf build

build/%.c: src/%.proto dirs
	protoc --c_out=$(@D) $(<F) --proto_path $(<D)
