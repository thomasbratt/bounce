# SDL2 is installed to the Linux system itself and is not a Cargo package.

install(){
    dnf install -y SDL2-devel
    dnf install -y SDL2_image-devel
    dnf install -y SDL2_gfx-devel
    dnf install -y SDL2_mixer-devel
    dnf install -y SDL2_ttf-devel  
}

main(){
    install
}

main "$@"

