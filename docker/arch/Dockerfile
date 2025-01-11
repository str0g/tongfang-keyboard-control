FROM archlinux:latest

RUN \
  pacman-key --init && \
  pacman -Syu --noconfirm --noprogressbar && \
  pacman -S --noconfirm --noprogressbar rust fakeroot debugedit git diffutils

RUN useradd -m --shell=/bin/bash build
