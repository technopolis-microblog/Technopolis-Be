# -*- mode: ruby -*-
# vi: set ft=ruby :

$provision = <<SCRIPT
  sudo apk update
  sudo apk add --no-cache build-base gcc libgcc libstdc++ llvm10-libs musl musl-dev libcrypto1.1 libcurl libgit2 libssl1.1 zlib libpq
  sudo apk add --no-cache redis postgresql postgresql-dev

  # Rustをインストール
  curl https://sh.rustup.rs -sSf | \
    sh -s -- --default-toolchain stable -y
  echo "source $HOME/.cargo/env" >> ~/.profile

  # 環境変数を読み込む
  source $HOME/.cargo/env

  # cargo-watchをインストール
  cargo install cargo-watch

  # データベースを起動
  sudo /etc/init.d/postgresql start
  sudo /etc/init.d/redis start
  
  cat << EOF | sudo su - postgres -c psql 
  -- Create the database user
  CREATE USER vagrant WITH PASSWORD 'password';

  -- Create the database
  CREATE DATABASE technopolis WITH OWNER=vagrant ENCODING='UTF8';
EOF
SCRIPT

$start = <<SCRIPT
    echo "Technopolis is ready. Let's roll!"
    echo "To server start:"
    echo '  $ vagrant ssh -c \"cd /srv/tp && cargo watch -x 'run' --clear\"'
SCRIPT

Vagrant.configure("2") do |config|
  config.vm.define "technopolis" do |config|
    # 基本的な設定
    config.vm.hostname = "technopolis"
    config.vm.box = "alpine-linux/alpine-x86_64"
    config.vm.provider "virtualbox" do |v|
      v.name = "tp_technopolis"
      v.memory = "2048"
    end

    # ファイル共有の設定
    config.vm.synced_folder "./", "/srv/tp", type: "rsync", rsync__exclude: [".git/"]
    config.vm.synced_folder ".", "/vagrant", disabled: true

    config.vm.network "forwarded_port", guest: 5432, host: 5432

    config.vm.provision :shell, inline: $provision, privileged: false
    config.vm.provision :shell, inline: $start, run: "always", privileged: false
  end
end
