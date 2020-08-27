set -e
sudo cargo install --path . --root /usr/local
sudo install -m644 50-daw.rules /etc/udev/rules.d
sudo udevadm control --reload
sudo udevadm trigger
echo ok
