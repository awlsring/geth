#!/bin/bash

sudo systemctl daemon-reload
sudo systemctl enable gethd.service
sudo systemctl start gethd.service