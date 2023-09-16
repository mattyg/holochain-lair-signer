# Holochain Lair Signer

This node library provides a convenient way to sign Holochain zome calls.

It uses napi-rs to wrap a native rust binary which calls the lair keystore via holochain_lair_api.

Note that napi-rs deseralizes incoming function names and arguments from camelCase to snake_case, and serializes the outgoing response to camelCase.

## Compatibility
This version is compatible with Lair Keystore 0.2.X

## Usage
```typescript
import { signZomeCallWithClient, ZomeCallUnsignedNapi, ZomeCallNapi } from "holochain-lair-signer";

export const signZomeCall = async (request: CallZomeRequest) => {

  const zomeCallUnsignedNapi ZomeCallUnsignedNapi = {
    provenance: Array.from(request.provenance),
    cellId: [Array.from(request.cell_id[0]), Array.from(request.cell_id[1])],
    zomeName: request.zome_name,
    fnName: request.fn_name,
    payload: Array.from(encode(request.payload)),
    nonce: Array.from(await randomNonce()),
    expiresAt: getNonceExpiration(),
  };

  const zomeCallSignedNapi: ZomeCallNapi =
    await signZomeCall(zomeCallUnsignedNapi);

  const zomeCallSigned: CallZomeRequestSigned = {
    provenance: Uint8Array.from(zomeCallSignedNapi.provenance),
    cap_secret: null,
    cell_id: [
      Uint8Array.from(zomeCallSignedNapi.cellId[0]),
      Uint8Array.from(zomeCallSignedNapi.cellId[1]),
    ],
    zome_name: zomeCallSignedNapi.zomeName,
    fn_name: zomeCallSignedNapi.fnName,
    payload: Uint8Array.from(zomeCallSignedNapi.payload),
    signature: Uint8Array.from(zomeCallSignedNapi.signature),
    expires_at: zomeCallSignedNapi.expiresAt,
    nonce: Uint8Array.from(zomeCallSignedNapi.nonce),
  };

  return zomeCallSigned;
};
```