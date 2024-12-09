import os

def day(n, example=True):
    folder = 'examples' if example else 'inputs'
    path = os.path.join('../data/', folder)
    fname = f'{n:02d}.txt'
    fpath = os.path.join(path, fname)
    return open(fpath, 'r').read().strip()
