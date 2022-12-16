# `RUST Devcontainer`

This Rust Development environment allows to work interactively with **VScode** on ML Projects using **Rust** and **devcontainers**.

**Prerequisites:** 

You must have the following software installed in your pc: 

For Linux based distributed Systems: 

- Docker 
- VsCode
    - Docker Extension
    - Dev Container Extensions

For Windows System:
- WSL2
- Docker Desktop
- VsCode
    - Docker Extension
    - Dev Container Extensions

"git clone" this repository and then, in order to reproduce the results initialize docker. 

If using Windows, then in WSL2:

```bat
sudo /etc/init.d/docker start
```

If using Linux based distributed System: 

```bat
sudo systemctl start docker
```
Remember to have added your user to the user group: 


```bat
sudo groupadd docker
sudo usermod -aG docker $USER
```
 
Then, create image from Docker file: 

```bat
sudo docker build -t rust:latest .devcontainer/
```

cd into the repository and then run interactive docker session, where "PWD" is your current working directory in the terminal:


```bat
sudo docker run -it --rm -p 8888:8888 -v "${PWD}":/home/ rust:latest
```

In Linux go to your vscode and open your working directoy, and press Crtl + Shift + p and select:

```bat
Dev containers: Attach to runnig container...
```

In Windows open Vscode then ress Crtl + Shift + p and select:

```bat
Attach to runnig container...
```

A new VsCode window will open up, now you can start working with jupyter files, python files, debuggers, etc. 