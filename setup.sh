#!/bin/bash

# evcxr setup. First insstall miniconda. 
# Then make sure it doesn't auto activate the base environment when a new terminal is created:
conda config --set auto_activate_base False
# cmake is a dependency on macOS for cargo install evcxr_jupyter
brew install cmake

# Setup the jupyter kernel. 
# Source: https://depth-first.com/articles/2020/09/21/interactive-rust-in-a-repl-and-jupyter-notebook-with-evcxr/
conda create --name evcxr
conda activate evcxr
conda install -y -c conda-forge nb_conda_kernels
cargo install evcxr_jupyter
evcxr_jupyter --install
jupyter notebook
# TODO: may want to consider installing jupyter lab with conda and using it instead
