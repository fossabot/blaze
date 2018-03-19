# project
PROJECT = $(shell pwd)

SOURCE = $(PROJECT)/src
TESTS  = $(PROJECT)/tests

# optimization
CORES  = $(shell nproc)

all: # Python 2.7
	cd $(SOURCE) && \
	   cargo build -j $(CORES) --verbose --all
	mv -v $(SOURCE)/target/debug/libblaze.so \
	      $(PROJECT)/blaze.so
	cp -v $(PROJECT)/blaze.so \
	      $(TESTS)/blaze.so

release: # Python 2.7
	cd $(SOURCE) && \
	   cargo build -j $(CORES) --release --all-features --verbose --all
	mv -v $(SOURCE)/target/release/libblaze.so \
	      $(PROJECT)/blaze.so
	cp -v $(PROJECT)/blaze.so \
	      $(TESTS)/blaze.so

docker-centos: # Centos 6.7
	sudo docker build \
	     --tag="blaze-centos:latest" \
	     "https://raw.githubusercontent.com/initbar/blaze/docker/centos/Dockerfile"
	sudo docker image rm "blaze-centos:latest"

docker-ubuntu: # Ubuntu 16.04
	sudo docker build \
	     --tag="blaze-ubuntu:latest" \
	     "https://raw.githubusercontent.com/initbar/blaze/docker/ubuntu/Dockerfile"
	sudo docker image rm "blaze-ubuntu:latest"

test:
	find $(TESTS) -type f -iname "test*.py" -exec pytest --benchmark-enable "{}" \;

clean:
	rm -rfv $(SOURCE)/target
	rm -fv $(PROJECT)/blaze.so
	rm -fv $(TESTS)/blaze.so
	find $(SOURCE) \
	     -type f \
	     -iname "*.py[pc]" \
	     -exec rm -fv "{}" \;
