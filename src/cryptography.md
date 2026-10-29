# Cryptography

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Calculate the SHA-256 digest of a file][ex-sha-digest] | [![ring-badge]][ring] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |
| [Sign and verify a message with an HMAC digest][ex-hmac] | [![ring-badge]][ring] | [![cat-cryptography-badge]][cat-cryptography] |
| [Salt and hash a password with PBKDF2][ex-pbkdf2] | [![ring-badge]][ring] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |
| [Calculate the SHA-256 digest of a file with sha2][ex-sha-digest-rustcrypto] | [![sha2-badge]][sha2] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |
| [Sign and verify a message with HMAC using hmac and sha2][ex-hmac-rustcrypto] | [![hmac-badge]][hmac] [![sha2-badge]][sha2] | [![cat-cryptography-badge]][cat-cryptography] |
| [Salt and hash a password with PBKDF2 using pbkdf2][ex-pbkdf2-rustcrypto] | [![pbkdf2-badge]][pbkdf2] | [![cat-cryptography-badge]][cat-cryptography] |
| [Salt and hash a password with Argon2][ex-argon2] | [![argon2-badge]][argon2] | [![cat-cryptography-badge]][cat-cryptography] |
| [Choose a hash algorithm at runtime][ex-dyn-digest] | [![sha2-badge]][sha2] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |
| [Hash data while writing it][ex-tee-hash] | [![sha2-badge]][sha2] [![data-encoding-badge]][data-encoding] | [![cat-cryptography-badge]][cat-cryptography] |
| [Upgrade stored password hashes on login][ex-rehash] | [![argon2-badge]][argon2] | [![cat-cryptography-badge]][cat-cryptography] |
| [Write code that is generic over the MAC][ex-generic-mac] | [![hmac-badge]][hmac] [![sha2-badge]][sha2] | [![cat-cryptography-badge]][cat-cryptography] |

[ex-sha-digest]: cryptography/hashing.html#calculate-the-sha-256-digest-of-a-file
[ex-hmac]: cryptography/hashing.html#sign-and-verify-a-message-with-hmac-digest
[ex-pbkdf2]: cryptography/hashing.html#salt-and-hash-a-password-with-pbkdf2
[ex-sha-digest-rustcrypto]: cryptography/hashing.html#calculate-the-sha-256-digest-of-a-file-with-sha2
[ex-hmac-rustcrypto]: cryptography/hashing.html#sign-and-verify-a-message-with-hmac-using-hmac-and-sha2
[ex-pbkdf2-rustcrypto]: cryptography/hashing.html#salt-and-hash-a-password-with-pbkdf2-using-pbkdf2
[ex-argon2]: cryptography/hashing.html#salt-and-hash-a-password-with-argon2
[ex-dyn-digest]: cryptography/extensible.html#choose-a-hash-algorithm-at-runtime
[ex-tee-hash]: cryptography/extensible.html#hash-data-while-writing-it
[ex-rehash]: cryptography/extensible.html#upgrade-stored-password-hashes-on-login
[ex-generic-mac]: cryptography/extensible.html#write-code-that-is-generic-over-the-mac

{{#include links.md}}
