import os

cwd = os.getcwd()
dirs = os.listdir()
env = os.getenv('STEP')
run_script = 'cargo check' if env == 'check' else 'cargo test'

for dir in dirs:
    full_path = os.path.join(cwd, dir)
    cargo_file = os.path.join(full_path, 'Cargo.toml')

    if os.path.isfile(cargo_file):
        os.chdir(full_path)

        if os.system(run_script):
            raise UserWarning('%s check failed' % full_path)