FROM ubuntu:17.10

RUN apt-get update
RUN apt-get install -y --no-install-recommends \
  gcc make libc6-dev git curl ca-certificates
RUN curl https://www.musl-libc.org/releases/musl-1.1.15.tar.gz | \
    tar xzf - && \
    cd musl-1.1.15 && \
    ./configure --prefix=/musl-x86_64 && \
    make install -j4 && \
    cd .. && \
    rm -rf musl-1.1.15 && \
# Install linux kernel headers sanitized for use with musl
    curl -L  https://github.com/sabotage-linux/kernel-headers/archive/v3.12.6-5.tar.gz | \
    tar xzf - && \
    cd kernel-headers-3.12.6-5 && \
    make ARCH=x86_64 prefix=/musl-x86_64 install -j4 && \
    cd .. && \
    rm -rf kernel-headers-3.12.6-5
ENV PATH=$PATH:/musl-x86_64/bin:/rust/bin
