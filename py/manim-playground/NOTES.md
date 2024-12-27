# Each time

```
conda-start
conda activate manim-playground
```

# Initial setup

## Getting Conda installed on my system

I'm trying out Conda for managing python dependencies here... generally following the 'vanilla' https://docs.anaconda.com/miniconda/ setup instructions

I skipped the conda shell setup (because I don't want the conda shell activation to run everytime i launch a shell, because it's pretty slow!). This is in my config.fish file:

```
function conda-start
    eval /home/logan/miniconda3/bin/conda "shell.fish" "hook" $argv | source
end
```

## Setting up the manim conda environment

See: https://docs.manim.community/en/stable/installation/conda.html

```
conda-start
conda create -n manim-playground
conda activate manim-playground
conda install -c conda-forge manim
```

Validate

```
command -v python
python --version
command -v manim
manim checkhealth
```
