# QSH (Quantum Shell)
Welcome to the repository for QSH! QSH is a program for remotely accessing a terminal on another device. It is cross-platform, and I even have plans to support WASM in the near future. The main ideas are simple:
* To use memory-safe, secure programming techiques.
* To use quantum-resistant encryption technologies.
* To make a modular and customizable program.
* To make something stable and consistent enough to use in a production environment.
---
## Why I'm making this at all:
This is just a high-school end-of-year project, but I also wanted to make something more extensible than the default OpenSSH.
## How It's Going to Work:
I'm not going to re-invent the wheel here, so I'm just going to stick with the basic outline of the SSH protocol, though this won't at all be compatible:
### Layers (lowest to highest):
1. Compression/Encryption:
    * Will be modular, so you can use whichever combination of encryption and compression you want, as long as it's implemented correctly :wink:
      Initially, only LZ4/AES-256 will be supported.
2. Authentication:
    * Also modular (like everything else here), only FIPS-204 will be supported initially. Will also contain some info about the session.
3. Metadata:
    * Information such as payload length, stream number (stdin, stdout, stderr), timestamp, etc.
4. Payload:
    * The actual payload (duh).
---
### Behavior:
#### Common:
Both client and server will have modular storage backends which allow for the storing of important information, like host address, keys for certification/asymetric encryption, last connection timestamp, etc.
#### Client:
On launch, the application will do several things:
1. It will check for an existing daemon; if one is found to be running, it will connect; otherwise, it will spin one up.
2. It will parse the command-line arguments, and ask the daemon to connect. The daemon will either establish a connection with the desired host, or if one already exists, and the transport layer supports multiplexing (like QUIC), it will simply spin up a new    stream.
3. Either way, it will now initiate a new session. Asymetric encryption (CRYSTALS-Kyber will be supported initially) is used to negotiate a symetric encryption key (AES will be supported initially) using a key exchange protocol (initially just Kyberlib's AKE    functionality) and a derivation function (HKDF will be supported initially). The encryption keys, both symetric and asymetric, will be unique to the session. Obviously, the certification keys will already exist (FIPS-204), and they will be loaded from the    storage backend as soon as the application is aware which host has been selected; While it negotiates the encryption stuff, the client will also attempt to negotiate other details, such as the compression algorithim.
4. Finally, the connection is ready for use, and the program will start forwarding I/O streams. Each packet will be compressed, signed, and encrypted.
#### Server:
When the server gets a connection request, it will start the process of begining a new session: exchanging symetric keys, authentication, negotiating compression and other details, etc as descibed above about the client side. Once everything is good and da   dandy, and the server knows who the client is and vice versa, the server will launch a shell as specified by the configuration file, and forward the relevant I/O streams to it. The server will _not_ close the session if the client adruptly disconnects; the client must explicity close the session.

---
# NOTE THAT THIS IS ALL SUBJECT TO CHANGE!
