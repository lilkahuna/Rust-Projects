#!/bin/bash

# Function to detect the operating system
detect_os() {
    case "$(uname -s)" in
        Darwin)
            echo "macOS"
            ;;
        Linux)
            echo "Linux"
            ;;
        *)
            echo "Unsupported OS"
            exit 1
            ;;
    esac
}   

# Print a message to the user
echo "Make sure the fsx executable is in the directory that this script is run from."
read -p "Press any key to continue..."


# Detect the operating system
os=$(detect_os)

# Set the binary name based on the OS
if [ "$os" = "macOS" ] || [ "$os" = "Linux" ]; then
    binary_name="fsx"
else
    echo "Unsupported OS"
    exit 1
fi

# Create the directory for fsx if it doesn't exist
sudo mkdir -p /usr/local/bin

# Get the current directory path
sourcepath="$(pwd)/$binary_name"

# Check if the binary exists
if [ ! -f "$sourcepath" ]; then
    echo "Error: $binary_name not found in the current directory."
    exit 1
fi

# Move fsx to the new directory
sudo mv "$sourcepath" /usr/local/bin

# Make the binary executable
sudo chmod +x /usr/local/bin/$binary_name

# Add /usr/local/bin to the PATH if it's not already there
if [[ ":$PATH:" != *":/usr/local/bin:"* ]]; then
    # Append to the shell profile for future sessions
    echo 'export PATH=$PATH:/usr/local/bin' >> ~/.profile
    echo 'Set user-wide environment variable'
else
    echo 'Directory already in PATH'
fi

# Export the PATH for the current session
export PATH=$PATH:/usr/local/bin

echo "fsx installation complete. Restart your terminal or run 'source ~/.profile' to update the PATH."
