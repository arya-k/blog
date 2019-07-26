# How I could code on ipadOS
<time>Jul 25, 2019</time>

With ipadOS coming out in 2019 just as I'm about to enter college, I'm extremely tempted to buy a brand new iPad pro. The battery life, minimal form factor and pencil support are compelling features for a college student. 

But there's one problem:

> **I like to code.**

And unfortunately, programming and iOS have never really gotten along. But times are changing, and I - as usual - am excessively optimistic. As a developer, I need the following:
 * Access to a filesystem
 * Ability to install packages/libraries
 * Ability to run any programming language I want.
 * A high quality code editor with keyboard shortcuts.

With ipadOS and access to a remote server, this dream finally seems within reach. Here's how I'd use [code-server](https://github.com/cdr/code-server) and [wireguard](https://github.com/wireguard/wireguard) to program on Apple's latest tablet:

### Lightsail instance

First, I need a remote server to run the editor in. These are instructions for AWS [lightsail](https://lightsail.aws.amazon.com). 

Create an instance with Ubuntu 18.04:

![Creating the lightsail instance](https://i.imgur.com/nsGLazs.png)

Pick an SSH key as appropriate, then pick the appropriate instance plan and name.

Next, create a Static IP by going to `Networking > Create Static IP`, and bind it to your instance:

![Binding to the instance](https://i.imgur.com/KhU9HRK.png)

Finally, forward the UDP port `54321`:

![Forwarding the UDP port](https://i.imgur.com/d3Ud3t1.png)

Press the terminal icon to connect to your instance!

![Pressing the terminal icon](https://i.imgur.com/DGBavBA.png)

### Updating the instance:

Begin by updating the instance with the latest and greatest packages from Ubuntu:

```bash
# escalate privileges
sudo -s

# update the OS
sudo apt-get update
DEBIAN_FRONTEND='noninteractive' apt-get -y -o Dpkg::Options::='--force-confdef' -o Dpkg::Options::='--force-confold' upgrade
DEBIAN_FRONTEND='noninteractive' apt-get -y -o Dpkg::Options::='--force-confdef' -o Dpkg::Options::='--force-confold' dist-upgrade
```

### Install Wireguard

We're going to use the Wireguard VPN for a fast, roaming, low latency connection to the instance.

First, install the relevant packages:
```bash
# install wireguard
echo iptables-persistent iptables-persistent/autosave_v4 boolean false | sudo debconf-set-selections
echo iptables-persistent iptables-persistent/autosave_v6 boolean false | sudo debconf-set-selections

add-apt-repository ppa:wireguard/wireguard -y
apt-get update
apt-get install wireguard qrencode -y
```

Then, generate public and private keys for the client and server:
```bash
# create keys
modprobe wireguard

cd /etc/wireguard
umask 077
wg genkey | tee server_private.key | wg pubkey > server_public.key
wg genkey | tee client_private.key | wg pubkey > client_public.key
```

Create the server config at `/etc/wireguard/wg0.conf`:
```bash
# create server config
echo "[Interface]
PrivateKey = server_private_key
Address = 10.0.0.1/24
ListenPort = 54321

[Peer]
PublicKey = client_public_key
AllowedIPs = 10.0.0.2/24" > wg0.conf

sed -i "s/server_private_key/$(sed 's:/:\\/:g' server_private.key)/" wg0.conf
sed -i "s/client_public_key/$(sed 's:/:\\/:g' client_public.key)/" wg0.conf
```

Then, create the client config at `/etc/wireguard/client.conf`:
```bash
# enable client key
echo "[Interface]
Address = 10.0.0.2/24
PrivateKey = client_private_key

[Peer]
PublicKey = server_public_key
Endpoint = server_ip:54321
AllowedIPs = 10.0.0.1/32" > client.conf

sed -i "s/client_private_key/$(sed 's:/:\\/:g' client_private.key)/" client.conf
sed -i "s/server_public_key/$(sed 's:/:\\/:g' server_public.key)/" client.conf
sed -i "s/server_ip/$(curl ipecho.net/plain)/" client.conf
```

Finally, start the service:
```bash
# start the service
systemctl enable wg-quick@wg0
systemctl start wg-quick@wg0
```

### Wireguard on ipadOS:

Now, install the wireguard client on your iPad, and setup the server connection using client.conf.

To ease the process, you can run the following:
```bash
qrencode -t ansiutf8 < /etc/wireguard/client.conf
```

Then, just scan the QR code on the iPad to setup the server.

### Installing code-server

Lastly, we have to setup code-server on the server:

```bash
# add code-server
export VERSION=$(curl --silent "https://api.github.com/repos/cdr/code-server/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

wget https://github.com/cdr/code-server/releases/download/$VERSION/code-server$VERSION-linux-x64.tar.gz -O code-server.tar.gz

tar -xvzf code-server.tar.gz
rm code-server.tar.gz
mv code-server*/code-server .

chmod +x ./code-server
rm -r ./code-server*-x64

sudo mv ./code-server /usr/local/bin/
```

Create a systemd service so it automatically runs whenever the server is restarted:

```bash
sudo echo "[Unit]
Description=Code-Server
After=network.target

[Service]
Type=simple
User=ubuntu
ExecStart=/usr/local/bin/code-server --allow-http --no-auth /home/ubuntu/code

[Install]
WantedBy=multi-user.target" > /etc/systemd/system/code-server.service
sudo systemctl enable code-server.service --now
```

### Usage

You're all set! Whenever you want to program on the iPad, connect wireguard to your VPN, and then just open up a browser and navigate to `http://10.0.0.1:8443`

![Coding on an iPad Pro](https://i.imgur.com/OL1iN01.mp4)

_Video from [this](https://medium.com/@ow/its-finally-possible-to-code-web-apps-on-an-ipad-pro-90ad9c1fb59a) medium article_