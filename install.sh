#!/bin/bash

config_path="$HOME/.config/dispatcher";
local_path="$HOME/.local";
data_path="$local_path/share/dispatcher";
model_path="$data_path/model";
local_bin_path="$local_path/bin";
profile_path="$config_path/profiles";

function remove_binaries() {
    destination=$(which dispatcher)
    if [ -z "$destination" ]; then
        echo "Dispatcher is not installed."
    else
        read -p "Remove $destination? (y/n) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            rm "$destination" || { echo "Could not remove file $destination"; exit 1; }
        else
            return 1
        fi
    fi
}

function uninstall() {
    if remove_binaries; then
        echo "Uninstallation completed. Please note config and model files are not removed."
    else
        echo "User aborted uninstallation."
    fi
}

function purge() {
    uninstall
    read -p "This will delete all profiles, models and data, continue y/n? " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$config_path" || { echo "Could not remove config path"; exit 1; }
        rm -rf "$data_path" || { echo "Could not remove data path"; exit 1; }
        echo "Purge completed. All dispatcher files removed."
    else
        echo "Purge aborted."
    fi
}

function installer() {
    cargo build || { echo "Failed to build dispatcher."; exit 1; }

    if [ ! -d "$config_path" ]; then
        mkdir -p "$config_path" || { echo "Failed to create config path"; exit 1; };
    else
        echo "Config Path Exists."
    fi

    if [ ! -d "$profile_path" ]; then
        mkdir -p "$profile_path" || { echo "Failed to create profile path"; exit 1; };
    else
        echo "Profile Path Exists."
    fi

    if [ ! -d "$data_path" ]; then
        mkdir -p "$data_path" || { echo "Failed to create data path"; exit 1; };
    else
        echo "Data Path Exists."
    fi

    if [ ! -d "$model_path" ]; then
        mkdir -p "$model_path" || { echo "Failed to create model path"; exit 1; };
    else
        echo "Model Path Exists."
    fi

    if [ -d "$local_bin_path" ]; then
        cp "target/release/dispatcher" "$local_bin_path"
    else
        echo "There is no local bin path detected. Do you want to install the binary to /usr/local/bin ?"
        read -p "y/n: " choice
        if [ "$choice" == "y" ]; then
            cp "target/release/dispatcher" "/usr/local/bin" || { echo "Could not install to /usr/local/bin, are you an admin?", exit 1; }
        fi
    fi

    echo "Installation completed."
    echo "You will need a Vosk model, please download it from https://alphacephei.com/vosk/models"
    echo "Then place it in a subfolder of $model_path . I recomend using a small model."
}

if [ "$#" -ge 1 ]; then
if [ "$1" == "uninstall" ]; then uninstall;
elif [ "$1" == "purge" ]; then purge;
else echo "Unknown parameter \"$1\""; fi
else
    installer
fi
