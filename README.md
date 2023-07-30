# Cringed

Cringed (Connectivity Relay Interfacing Numerous, Goofy ESPs - daemon) has been created for easier interfacing with embedded peripherals from the userspace layer.

Applications can easily interface with buttons, encoders, displays and such. ESPs can also be used as bluetooth bridge / ADV packet listener (things developers do to avoid bluez aimritefolks)

The project consists of 2 components:

## 1.Cringed-Server

Creating various interfaces for external applications:
* socket.io (IN PROGRESS)
* Web client (IN PROGRESS - mock app for testing)
* MQTT (TODO)
* REST API (TODO)
* Some simple interface for bash scripts
* Creating virtual HID device for volume control, custom gaming controllers, etc (TODO)
* more?


It's written in rust because I can't get laid

## 2.Cringedge

The project uses blazingly-fast config files to spawn a cringedges (cringed-bridges) for any appearing and cringed-compatible ESP device.

(see utils/ directory for more)

After serial connection is estabilished, the cringedge communicates with ESP using flatbuf-packed messages. The capabilities are propagated to the server
via socket.io

The device can easily send runtime logs that can be stored on the host device and viewed live by remote clients

Cringedges can also perform a firmware update on their ESP companions easily, using esptool as a backend.

# Usecases

* Cheapo 16x2 alphanumeric displays + ESP can serve as a status display for a server.
* Attaching peripherals to SMBCs - I'd prefer destroying a random ESP from a wiring error, than a rasberry pi. 
* Attaching peripherals to PCs.
* (tbc.) 


### Useful links

https://documentation.suse.com/sles/12-SP4/html/SLES-all/cha-udev.html
http://blog.fraggod.net/2015/01/12/starting-systemd-service-instance-for-device-from-udev.html
http://0pointer.de/blog/projects/instances.html
https://askubuntu.com/questions/49910/how-to-distinguish-between-identical-usb-to-serial-adapters