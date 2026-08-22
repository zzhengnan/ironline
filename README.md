# ironline

A linear regression library implemented in Rust.

## Development

```
# Create dev env
conda env create -f ironline.yml -n ironline
conda activate ironline

# Install Python wrapper into current env (via pip)
maturin develop

>>> import ironline
>>> ironline
<module 'ironline' from '~/miniforge3/envs/ironline/lib/python3.14/site-packages/ironline/__init__.py'>
>>> ironline.compute_mean([1, 3, 4])
2.6666666666666665
```
