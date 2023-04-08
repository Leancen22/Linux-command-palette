# Terminal interactiva basada en comandos unix.

# Comandos pendientes:
- [x] cd
- [x] ls
- [ ] mkdir
- [ ] rm
- [ ] rm -r
- [ ] help
- [ ] mv
- [ ] cp
- [ ] cat
- [ ] touch 
- [ ] pwd

# Instalacion del proyecto:

    git clone ...
    
    cargo build
    
# PATH Global

Esto generara un release en /target/release

    cargo build --release
    
Aqui se agrega al PATH, para saber /ruta/al/proyecto usar "pwd"

    export PATH="$PATH:/ruta/al/proyecto/target/release"

En sistemas windows:

    setx PATH "%PATH%;C:\ruta\al\proyecto\target\release"

Ejecutando:

    console_commands

Se puede usar el programa en cualquier lugar.
    
# Dependencias externas:

Como se ve en las dependencias del archivo Cargo.toml, momentaneamente solo se tiene como
dependencia externa a Rust:

    termion = "2.0.1"
