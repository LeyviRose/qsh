# QSH (Quantum Shell)
Welcome to the repository for QSH! QSH is a program for remotely accessing a terminal on another device. It is cross-platform. The main ideas are simple:
* To use memory-safe, secure programming techniques.
* To use quantum-resistant encryption technologies.
* To make a modular and customizable program.
* To make something stable and consistent enough to use in a production environment.
---
## Why I'm making this at all:
This is just a high-school end-of-year project, but I also wanted to make something more extensible than the default OpenSSH.
## How It's Going to Work:
I'm not going to re-invent the wheel here, so I'm just going to stick with the basic outline of the SSH protocol, though this won't at all be compatible:
### Layers (lowest to highest):
#### Transport
By default, TCP encrypted with AES-GCM-256 (for privacy in higher layers), keys are exchanged with CRYSTALS-Kyber.
#### Session
Default is authenticated with CRYSTALS-Dilithium, encrypted with AES-GCM-256, and does not support forwarding. 
#### Channel
For now, only compression with LZ4 is supported.

---
### Behavior:
#### Common:
Both client and server will have modular storage backends which allow for the storing of important information, like host address, keys for certification/asymetric encryption, last connection timestamp, etc.
#### Client:
On launch, the application will do several things:
1. It will check for an existing daemon; if one is found to be running, it will connect; otherwise, it will spin one up.
2. It will parse the command-line arguments, and ask the daemon to connect. The daemon will either establish a connection with the desired host, or if one already exists, and the transport layer supports multiplexing (like QUIC), it will simply spin up a new    stream.
3. Either way, it will now initiate a new session. Asymmetric encryption (CRYSTALS-Kyber will be supported initially) is used to negotiate a symetric encryption key (AES will be supported initially) using a key exchange protocol (initially just Kyberlib's AKE    functionality) and a derivation function (HKDF will be supported initially). The encryption keys, both symetric and asymetric, will be unique to the session. Obviously, the certification keys will already exist (FIPS-204), and they will be loaded from the    storage backend as soon as the application is aware which host has been selected; While it negotiates the encryption stuff, the client will also attempt to negotiate other details, such as the compression algorithim.
4. Finally, the connection is ready for use, and the program will start forwarding I/O streams. Each packet will be compressed, signed, and encrypted.
#### Server:
When the server gets a connection request, it will start the process of beginning a new session: exchanging symmetric keys, authentication, negotiating compression and other details, etc as described above about the client side. Once everything is good and dandy, and the server knows who the client is and vice versa, the server will launch a shell as specified by the configuration file, and forward the relevant I/O streams to it. The server will _not_ close the session if the client adruptly disconnects; the client must explicity close the session.

---
# NOTE THAT THIS IS ALL SUBJECT TO CHANGE!
