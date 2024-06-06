import docker
import os
import shutil


print ("Running")
# Initialize Docker client
client = docker.from_env()


local_directory = './repository'
container_directory = '/artifact'
file_name = 'test1.result.json'

# Create the local directory if it doesn't exist
os.makedirs(local_directory, exist_ok=True)
def makefile():
    
    # Run Docker container to generate the JSON file
    print("Running Docker container...")
    container = client.containers.run(
        "ghcr.io/little-bear-labs/lbl-test-proxy:latest",
        "sh -c 'echo {\"key\": \"value\"} > /artifact/test1.result.json'",
        volumes={os.path.abspath(local_directory): {'bind': container_directory, 'mode': 'rw'}},
        detach=True
    )
    container.wait()  # Wait for the container to finish

    # Check if the file exists and its size
    file_path = os.path.join(local_directory, file_name)
    if os.path.exists(file_path):
        file_size = os.path.getsize(file_path)
        print(f"File {file_name} saved successfully with size {file_size / (1024 * 1024):.2f} MB")
    else:
        print(f"File {file_name} not found.")

    # Cleanup container
    container.remove()
