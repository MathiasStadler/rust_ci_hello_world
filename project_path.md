https://dev.to/mark_mwendia_0298dd9c0aad/optimizing-devops-pipelines-for-rust-projects-leveraging-cargo-and-cicd-474d

# project path
<!-- keep the format -->
## Create for your own project a new project folder in the console(terminal, bash shell),  e.g. in your own home folder,  and open it as a new project inside your program used - in my case MS VSCode / MS VSCodium
<!-- To comply with the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
project_name="new_project"
echo ${project_name} 
# cd && mkdir <project_name> folder> && cd $_
# command 'cd' change to home folder from logged in user
# command 'mkdir' create the DIRECTORY(ies), if they do not already exist
# command `cd` <folder>`change to the folder
# command '$_' last argument of last command
cd && mkdir ${project_name} && cd $_
```
<!-- keep the format -->
## Create a new rust based project inside the previously generated folder and update the rust binary's system wide to the last stable version
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
touch README.md \
&& ln -s README.md README \
&& cargo init "." \
&& cargo add rustfmt \
&& rustup component add rustfmt \
&& mkdir examples \
&& cp src/main.rs examples/example.rs \
&& sed -i -e 's/world/example/g' examples/example.rs \
&& rustup show \
&& rustup check \
&& rustup toolchain uninstall stable \
&& rustup toolchain install stable \
&& rustup update  --force \
&& mkdir tests \
&& rustup override set stable \
&& rustup show |sed -n '/active toolchain/,/^$/p' 

```

<!-- keep the format -->
## Show which toolchain is active
<!-- keep the format -->
```bash <!-- markdownlint-disable-line code-block-style -->
rustup show
# or better
rustup show |sed -n '/active toolchain/,/^$/p'
```
<!-- keep the format -->