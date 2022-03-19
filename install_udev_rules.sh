set -e

sudo install -m644 50-daw.rules /etc/udev/rules.d
sudo udevadm control --reload
sudo udevadm trigger
echo ok
