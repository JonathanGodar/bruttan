# Goal

This program is supposed to make playing different maps on a minecraft server really easy. By easy we mean

1. One should only have to upload a map and the server will start with a working map.

The goal of this program is to provide a minecraft realm like experience for a normal minecraft server.

## Requirements

To fufill 1. we need to:

- Identify the version of the map and start a server with the correct version
- Add plugins so that a client of (almost) any version can play on the server. Backward compatibility is most important, meaning one should use the newest client.
- Have a web interface where maps can be uploaded and the server(s) can be managed.

## Definitions

Map:

- A collection of files which together are able to make a playable minecraft mini-game/"map". This includes any required resource pack

Map instance:

- A collection of files which is a map which has been ran by the server. It is thus possible to create mutliple map instances from a map where each instance corresponds to a play-through of the map.

Minecraft Server Spec:

-

Minecraft Server:

- A program which takes map instances and is able to run them as per the game mechanics of minecraft.

## Arkiteture (WIP)

- Podman to
-

## Planned features

- Map storage
- Ability to have multiple map instances.

## Possible features
