# Dynamic NFT

## What is Dynamic NFT?

Dynamic NFT is a project that let you create collections of NFTs that are specific to an event (like HackWasm Berlin 2023) and represents badges of attendace to that.

The innovative feature is that every collection has a whitelist of addresses, like judges in a hackaton, that can in a specific timeframe (e.g. a week) turn metadata for only selected holders of the badge. This is what makes NFTs dynamics. Considering the hackaton example, our implementation allows to turn metadata for only winners of the competition, having a different info with respect to standard participant while beeing in the same collection.

Every event has a specific collection tied to it, and every badge issued shares the same metadata. Metadata are not saved into every badge (it would be a waste of space) but into the collection itself. The key is that we modify the normal query so that when asking for the nft metadata of a specific badge, it checks if it has some special role (e.g. winner of hackaton) and returns the corresponding metadata.

## Definitions

Below you can find relevant definitions required to understand this specification.

- `dyn-nft`: it's our wrapper around sg-721, which itself is a custom implementation of cw-721. It represents a collection of dynamic badges.
- `factory`: the contract that instantiate `dyn-nft` and is the only address allowed to change the state of this contract.

## Technical Specifications

### Dyn-nft

This is the most important contract. It represents our custom wrapper around sg-721 that saves metadata on the collection itself.

It also saves a map that can be updated to track dyn-nft with special role.

```typescript
interface Dyn-nft {
    TurnMetadata,
}
```

Dyn-nft also has all normal messages with corresponding handler functions of standard sg-721.

Below is a brief description of the logic that should be implemented for the interface's methods:

```typescript
function TurnMetadata(token_id, role) {
    // only factory
    // change the metadata reference to the selected badge
}
```

### Factory

The factory is the main entry point for a user that wishes to create a new collection of dynamic badges tied to a specific event.

```typescript
interface Factory {
    CreateDynNft,
    MintBadge,
    SetWhitelist,
    TurnMetadata,
    FreezeContract
}
```

```typescript
function CreateDynNft(event_info) {
    // create a new collection of badges tied to a specific event
}
```

```typescript
function MintBadge() {
    // only whitelisted addresses for specific collection
    // check not freezed
    // mint one badge-nft token
}
```

```typescript
function SetWhitelist(collection) {
    // only once per collection
    // create a whitelist of addresses that can later call `TurnMetadata` on the collection.
}
```

```typescript
function TurnMetadata(collection) {
    // only whitelisted address
    // check not freezed
    // call the underlining function on colletcion
    // change the metadata reference to the selected badge
}
```

```typescript
function FreezeContract() {
    // only whitelisted address
    // after that metadata cannot be changed anymore
}
```

## Desired Properties

- Every event has its specific `dyn-nft` collection.
- Metadata are saved into the collection `dyn-nft` itself and not on every badge or in a hub contract.
- After a specific time, dynamic badges cannot change their state anymore.
