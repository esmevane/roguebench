# Network Connection

Managing client-server connections including connect, disconnect, and reconnect flows.

## Core Logic

**State**

- Connection state (disconnected, connecting, connected, disconnecting)
- Session token (for reconnection)
- Client ID (server-assigned)
- Round-trip time (RTT)

**Connection States**

```
Disconnected → Connecting → Connected → Disconnecting → Disconnected
                    ↓            ↓
                 Failed     Reconnecting
```

**Operations**

`connect(address)` - Initiate connection

- Begin handshake with server
- Transition to Connecting state
- Timeout after duration

`disconnect()` - Clean disconnect

- Notify server
- Clean up local state
- Transition to Disconnected

`reconnect(token)` - Resume session

- Attempt to restore previous session
- Server validates token
- Resync state if successful

**Server Operations**

`accept(client)` - Accept new connection

- Assign client ID
- Generate session token
- Add to connected clients

`kick(client, reason)` - Force disconnect

- Notify client of reason
- Remove from connected list
- Clean up client state

**Invariants**

- One connection per client
- Session tokens expire after timeout
- Reconnect restores player entity if within window
- Clean disconnect preferred over timeout

**Defaults**

| Field | Value | Description |
|-------|-------|-------------|
| Connect timeout | 10s | Max connection time |
| Reconnect window | 30s | Session token validity |
| Heartbeat | 1s | Keep-alive interval |

---

## Bevy Integration

**Resources**

- ConnectionState enum
- SessionToken(Option<String>)
- ClientId(Option<u64>)
- ServerAddress(String)

**Events**

- ConnectRequest { address }
- Connected { client_id }
- Disconnected { reason }
- ConnectionFailed { error }

**Systems**

- Handle connection state machine
- Send/receive heartbeats
- Detect timeout
- Process reconnection

**UI Integration**

- Connection status indicator
- Disconnect reason display
- Reconnecting overlay
