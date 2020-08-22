FROM node:buster

ENV HOME=/home/root
WORKDIR $HOME

# Required dependencies
RUN apt-get update \
    && apt-get install -y \
        curl \
        make \
        protobuf-compiler \
        python3-pip \
    && apt-get autoremove -y \
    && apt-get clean

# Cargo + Rust
ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup \
    PATH=/usr/local/cargo/bin:$PATH
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && cargo install wasm-pack wasm-bindgen-cli \
    && rm -r /usr/local/cargo/registry

# Google Chrome
RUN wget -q -O - https://dl-ssl.google.com/linux/linux_signing_key.pub | apt-key add - \
    && echo "deb http://dl.google.com/linux/chrome/deb/ stable main" >> \
        /etc/apt/sources.list.d/google.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends google-chrome-stable

# Rollup
RUN npm install -g rollup

# Python dependencies
COPY requirements.txt .
RUN ln -s /usr/bin/python3 /usr/local/bin/python \
    && ln -s /usr/bin/pip3 /usr/local/bin/pip \
    && pip install --no-cache-dir -r requirements.txt \
    && rm requirements.txt

ENTRYPOINT []
CMD ["/bin/bash"]
