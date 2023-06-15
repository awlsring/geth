#!/bin/bash

SRC_DIR=$(git rev-parse --show-toplevel)

sudo groupadd gethd
sudo useradd -r -g gethd -s /sbin/nologin -d /etc/gethd gethd
sudo chown -R gethd:gethd /etc/gethd/

mkdir -p /opt/gethd
sudo chown -R gethd:gethd /opt/gethd
sudo chmod 755 /opt/gethd
cp $SRC_DIR/apps/agent/resources/gethd.service /etc/systemd/system/gethd.service