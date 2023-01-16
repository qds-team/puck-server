import subprocess
import os
import shutil

SERVER_DIR_NAME = 'server'
CONFIG_CLIENT_DIR_NAME = 'config-client'
BUILD_DIR_NAME = 'build'

# resolve paths
mydir = os.path.dirname(os.path.abspath(__file__))
server_dir = os.path.join(mydir, SERVER_DIR_NAME)
config_client_dir = os.path.join(mydir, CONFIG_CLIENT_DIR_NAME)
build_dir = os.path.join(mydir, BUILD_DIR_NAME)

# delete build directory
if os.path.isdir(build_dir):
	shutil.rmtree(build_dir)

# build server and config client
server_build = subprocess.Popen(f'cargo build --manifest-path="{server_dir}/Cargo.toml"', shell=True)
config_client_build = subprocess.Popen(f'yarn --cwd "{config_client_dir}" run build', shell=True)

# wait for both builds to finish
server_build.wait()
config_client_build.wait()

# move files to build directory
subprocess.run(f'mv "{server_dir}/target" "{build_dir}"', shell=True)
subprocess.run(f'mv "{config_client_dir}/build" "{build_dir}/static"', shell=True)
