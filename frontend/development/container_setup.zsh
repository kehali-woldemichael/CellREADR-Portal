#!/bin/zsh

# allows wildcard characters to include dotfiles
# General
# Setup enviroment ... 

# Rust 
#curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
#  echo '. "$HOME/.cargo/env"' >> /home/.zshrc && \
#  source .zshrc && \
#  cargo install zoxide eza && \

# symlink files in Core/home to root directory 
for file in /root/Core/home/*(D); do 
  ln -sf $file /root
done

./root/Core/apps/fzf/install

# Install rust
#curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# To keep container running
#tail -f /dev/null
sleep infinity
