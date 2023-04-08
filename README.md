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

# Instalacion del proyecto:

    git clone ...
    
    cargo build
    
# Dependencias externas:

Como se ve en las dependencias del archivo Cargo.toml, momentaneamente solo se tiene como
dependencia externa a Rust:

    termion = "2.0.1"
