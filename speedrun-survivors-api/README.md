## Speedrun Survivors API server

### Helius cNFT API
- interface with helius to mint, burn, list, ... cNFTs

### Client requires a wallet
- generate a hot wallet by default so players can seemingly play without

### Client Requests
- NFT_LIST
  - client data:
    - pubkey
  - server:
    - return a list of all NFTs owned by that pubkey (so they can be equipped at the start of the game)
  - (later)(possible abuse)
    - add the current unixtime and sign the request
    - server checks unixtime is somewhat current; verifies signature
- SESSION_GET
  - client data:
    - pubkey
  - server:
    - info about the current session if it exists
      - status
      - entropy
- SESSION_INIT
  - client data:
    - pubkey (either actual wallet; or some in game generated hot wallet)
  - conditions:
    - if DB has an entry for pubkey that is in STATE:started
      - check if timestamp is older than 1h
        - overwrite data if it is
      - fail if it isn't (client then needs a cancel request with signature to remove data and start a new game)
  - server:
    - generate entropy for seeding the RNG on the client (u256; string: 44 bytes as base64)
    - store in DB pubkey -> {entropy: String; state: (pending; started); timestamp: u64}
    - this allows only 1 valid session per pubkey (to somewhat prevent sharing NFTs)
- GAME_START
  - client data:
    - pubkey
    - entropy
    - signature of "PUBKEY:ENTROPY"
  - update the state & timestamp in the DB
  - server:
    - returns result status
- GAME_COMPLETE
  - client data:
    - pubkey
    - list of equipped NFT ids (hero, items, buffs, ...)
    - the initial RNG bytes
    - the replay data
    - a signature over NFT + RNG + REPLAY data
  - verify DB game time did not exceed 1 hour
  - verify DB game state was started
  - verify DB fame state matches provided entropy
  - (later) verify the signature
  - (later) verify the u256 RNG has never been used before (PREVENT replay attacks)
  - (later) verify player owns the NFTs
  - (later) verify the replay integrity; issue cNFT according to what should the replay says
  - for now just issue some random cNFTs / or whatever the client game requests
  - need to take a signature of something random from client to prove possession of the private key

### TODO
can users transfer cNFTs on their own or do they need to be minted?
above doesnt protect pubkey starting game with some nfts and transferring them to other pubkey.. as it only checks ownership when the came concludes

### Testing certs
```
mkdir cert
cd cert
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -days 365 -sha256 -subj "/C=CN/ST=Fujian/L=Xiamen/O=TVlinux/OU=Org/CN=muro.lxd"
openssl rsa -in key.pem -out nopass.pem
rm key.pem
mv nopass.pem key.pem
```