# Pica

Pica is a virtual UWB Controller implementation supporting UWB ranging sessions.
It has been designed for testing UWB ranging capabilities.
Pica supports the following features:

- Pica keeps an internal representation of a 3-D scene.
- Pica lets multiple clients connect through TCP sockets.
  Each new connection spawns an attached UWB subsystem. Connected hosts can
  interact together as if they existed in a single 3-D scene.
- Pica implements a nice GUI through a web server.
- Pica provides additional vendor commands to create and configure
  virtual beacons.

# Architecture

- *Device* UWB subsystem created for a connected host.
- *Session* UWB ranging session opened by a connected host.
- *Beacon* virtual UWB host, responding to ranging requests from
  connected hosts.

```
                 ┌────────────────────┐
                 │ Web                │
                 │                    │
                 └─────┬─────────▲────┘
                       │         │    HTTP localhost:3000
  ┌────────────────────▼─────────┴───────┐
  │                                      │
  │                 Picaaaaaaaaaa        │
  │                                      │
  │  ┌────────┐  ┌────────┐  ┌────────┐  │
  │  │Beacon1 │  │Device1 │  │Device2 │  │
  │  ├────────┤  │        │  │        │  │
  │  │Beacon2 │  ├────────┤  ├────────┤  │
  │  ├────────┤  │Session1│  │Session1│  │
  │  │...     │  ├────────┤  ├────────┤  │
  │  │        │  │Session2│  │Session2│  │
  │  └────────┘  └──▲──┬──┘  └──▲──┬──┘  │
  │                 │  │        │  │     │
  └─────────────────┼──┼────────┼──┼─────┘
                    │  │        │  │  TCP localhost:7000
                 ┌──┴──▼──┐  ┌──┴──▼──┐
                 │Client1 │  │Client2 │
                 │        │  │        │
                 ├────────┤  ├────────┤
                 │VirtIO  │  │        │
                 ├────────┤  │        │
                 │UWB HAL │  │        │
                 ├────────┤  │Python  │
                 │Cuttle  │  │console │
                 │fish    │  │        │
                 └────────┘  └────────┘
```

# Vendor commands

Pica implements vendor commands in the group `0x09` to let hosts report their
MAC address and current position. Other commands allow the hosts to create and
modify beacons.

## Position

The position encodes the physical localization and orientation of an UWB
device.

| Position Fields | Length   | Value/Description                               |
|-----------------|----------|-------------------------------------------------|
| X               | 2 Octets | X Coordinate                                    |
| Y               | 2 Octets | Y Coordinate                                    |
| Z               | 2 Octets | Z Coordinate                                    |
| Azimuth         | 2 Octets | Azimuth                                         |
| Elevation       | 1 Octets | Elevation                                       |

## PICA_INIT_DEVICE_CMD (`0x00`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Mac Address    | 8 Octets | Replace the generated mac address for the UWB subsystem. The default mac address is a counter  incremented for  each new connection. |
| Position       | 9 Octets | Report the initial position of the UWB device.   |

## PICA_INIT_DEVICE_RSP (`0x00`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Status         | 1 Octet  | Status code                                      |

## PICA_SET_DEVICE_POSITION_CMD (`0x01`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Position       | 9 Octets | Report the current position of the UWB device.   |

## PICA_SET_DEVICE_POSITION_RSP (`0x01`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Status         | 1 Octet  | Status code                                      |

## PICA_CREATE_BEACON_CMD (`0x02`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Mac Address    | 8 Octets | Selected mac address for the UWB beacon.         |
| Position       | 9 Octets | Report the initial position of the UWB beacon.   |

## PICA_CREATE_BEACON_RSP (`0x02`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Status         | 1 Octet  | Status code                                      |

## PICA_SET_BEACON_POSITION_CMD (`0x03`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Mac Address    | 8 Octets | Mac address of the UWB beacon to edit.           |
| Position       | 9 Octets | Report the current position of the UWB beacon.   |

## PICA_SET_BEACON_POSITION_RSP (`0x03`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Status         | 1 Octet  | Status code                                      |

## PICA_DESTROY_BEACON_CMD (`0x04`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Mac Address    | 8 Octets | Mac address of the UWB beacon to remove.         |

## PICA_DESTROY_BEACON_RSP (`0x04`)

| Payload Fields | Length   | Value/Description                                |
|----------------|----------|--------------------------------------------------|
| Status         | 1 Octet  | Status code                                      |
