This document describes important things to keep notice off when working with game networking. 
First some terms shortly described what they are and related links to those terms. 
Whereafter a [section](https://github.com/TimonPost/amethyst/blob/net/amethyst_network/README.md#networking-from-other-engines) follows about existing networking implementations and things we can learn from them an thinks we can do better.

Table of contents:
1. [Sockets](https://github.com/TimonPost/amethyst/blob/net/amethyst_network/README.md#sockets)
2. [Congestion Avoidance](https://github.com/TimonPost/amethyst/blob/net/amethyst_network/README.md#congestion-avoidance--more)
3. [Other Engines](https://github.com/TimonPost/amethyst/blob/net/amethyst_network/README.md#sockets)

# Sockets
When developing a network implementation for a game engine we need to keep a couple of things in mind. 
The important things are noted below. 

The most text you see here comes from this [site](https://gafferongames.com/categories/game-networking/) but it is somewhat shortened and less detailed. 
I have done this so we can have a basic picture of what is done to avoid to much details at once. 

The cool thing is that there is already a rust [crate](https://github.com/acmcarther/gaffer_udp) implementing all the best practise [gaffer](https://gafferongames.com/categories/game-networking/) describes in his blogs. 
The only thing that needs to be done is Congestion Avoidance (See below). So whe could use that crate or some parts of it for or own use. See this for some [conversation](https://github.com/acmcarther/gaffer_udp/issues/10) I had with the developer of the crate.

## Why UDP and not TCP | [More](https://gafferongames.com/post/udp_vs_tcp/)
Those of you familiar with TCP know that it already has its own concept of connection, reliability-ordering and congestion avoidance, so why are we rewriting our own mini version of TCP on top of UDP?

The issue is that multilayer action games rely on a steady stream of packets sent at rates of 10 to 30 packets per second, and for the most part, the data contained is these packets is so time sensitive that only the most recent data is useful.
This includes data such as player inputs, the position, orientation and velocity of each player character, and the state of physics objects in the world.

The problem with TCP is that it abstracts data delivery as a reliable ordered stream. Because of this, if a packet is lost, TCP has to stop and wait for that packet to be resent.
This interrupts the steady stream of packets because more recent packets must wait in a queue until the resent packet arrives, so packets are received in the same order they were sent.

What we need is a different type of reliability.
Instead of having all data treated as a reliable ordered stream, we want to send packets at a steady rate and get notified when packets are received by the other computer.
This allows time sensitive data to get through without waiting for resent packets, while letting us make our own decision about how to handle packet loss at the application level.

What TCP does is maintain a sliding window where the ack sent is the sequence number of the next packet it expects to receive, in order. 
If TCP does not receive an ack for a given packet, it stops and resends a packet with that sequence number again. This is exactly the behavior we want to avoid!

It is not possible to implement a reliability system with these properties using TCP, so we have no choice but to roll our own reliability on top of UDP.
reliability for udp,

## When TCP
Of course there could be cases we could do over TCP like credits, chat etc. We can setup a TCP socket for this asside UDP. 
But we also could make or UDP channel reliable as described below so when we detect package lost on the client we could construct a new package    

## Other protocols
There are other protocols like that could be helpful so we need to make it easy to witch protocol.
If you have an idea please add your protocol and describe why it would be useful.

## Reliability

Of course we want to have a reliable stream of packets not and since UDP isn't reliable and in order we have to use some features from TCP and add those to or own implementation.

### Sequence Numbers
So first we care about the order of which the packages arrive. 
This is actually quite a common technique. 
It’s even used in TCP! These packet ids are called sequence numbers.
 
We can accomplice this by adding a sequence number to every package so that we know when the other computer receives a packet it knows its sequence number according to the computer that sent it.

### Acknowledgements
We want to let the client know if a package is received. 
This we can do by sending acknowledgements. 
When a package arrives at the server we send an acknowledgement back with the sequence number of that package.
Now that we know what packets are received by the other side of the connection, how do we detect packet loss?
The trick here is to flip it around and say that if you don’t get an ack for a packet within a certain amount of time, then we consider that packet lost.

What TCP does is maintain a sliding window where the ack sent is the sequence number of the next packet it expects to receive, in order. 
If TCP does not receive an ack for a given packet, it stops and resends a packet with that sequence number again. 
This is exactly the behavior we want to avoid since we don't care in the most times about old data!

## Congestion Avoidance | [More](https://gafferongames.com/post/reliability_ordering_and_congestion_avoidance_over_udp/)

If we just send packets without some sort of flow control, we risk flooding the connection and inducing severe latency (2 seconds plus!) as routers between us and the other computer become congested and buffer up packets.
This happens because routers try very hard to deliver all the packets we send, and therefore tend to buffer up packets in a queue before they consider dropping them.

We need to focus on what we can actually do which is to avoid flooding the connection in the first place. 
We try to avoid sending too much bandwidth in the first place, and then if we detect congestion, we attempt to back off and send even less.

### Round trip time [RTT](https://en.wikipedia.org/wiki/Round-trip_delay_time)
Round-trip time (RTT) is the length of time it takes for a signal to be sent plus the length of time it takes for an acknowledgement of that signal to be received.

We need a way to measure the RTT of our connection.

Here is the basic technique:

 -   For each packet we send, we add an entry to a queue containing the sequence number of the packet and the time it was sent.

 -  Each time we receive an ack, we look up this entry and note the difference in local time between the time we receive the ack, and the time we sent the packet. This is the RTT time for that packet.

 -  Because the arrival of packets varies with network jitter, we need to smooth this value to provide something meaningful, so each time we obtain a new RTT we move a percentage of the distance between our current RTT and the packet RTT. 10% seems to work well for me in practice. This is called an exponentially smoothed moving average, and it has the effect of smoothing out noise in the RTT with a low pass filter.

 -  To ensure that the sent queue doesn’t grow forever, we discard packets once they have exceeded some maximum expected RTT. As discussed in the previous section on reliability, it is exceptionally likely that any packet not acked within a second was lost, so one second is a good value for this maximum RTT.

Now that we have RTT, we can use it as a metric to drive our congestion avoidance. 
If RTT gets too large, we send data less frequently, if its within acceptable ranges, we can try sending data more frequently.

We have to define to network conditions, `Good` and `Bad`. 
When network conditions are `Good` we send 30 packets per-second, and when network conditions are `Bad` we drop to 10 packets per-second.

When in bad modes try to recover to good modes and when in good modes try too advance and send more data if possible.

## Virtual connections | [More](https://gafferongames.com/post/virtual_connection_over_udp/)
It is handy to keep track of connected clients. 

If you have used TCP sockets then you know that they sure look like a connection, but since TCP is implemented on top of IP, and IP is just packets hopping from computer to computer, it follows that TCP’s concept of connection must be a virtual connection.

If TCP can create a virtual connection over IP, it follows that we can do the same over UDP.

Lets define our virtual connection as two computers exchanging UDP packets at some fixed rate like 10 packets per-second. 
As long as the packets are flowing, we consider the two computers to be virtually connected.

### Protocol id
Since UDP is connectionless our UDP socket can receive packets sent from any computer.

We’d like to narrow this down so that the server only receives packets sent from the client, and the client only receives packets sent from the server. 
We can’t just filter out packets by address, because the server doesn’t know the address of the client in advance.

The protocol id is just some unique number representing our game protocol. 
Any packet that arrives from our UDP socket first has its first four bytes inspected. 
If they don’t match our protocol id, then the packet is ignored. 
If the protocol id does match, we strip out the first four bytes of the packet and deliver the rest as payload. 

# Networking from other engines
In this section we can discuss the good and bad parts about networking systems from other engines. 
 
## Networking Unity | [check](https://docs.unity3d.com/Manual/UNet.html)
Here the important things of unity networking could be worked out.

## Networking Unreal | [check](https://docs.unrealengine.com/en-us/Gameplay/Networking)
Here the important things of unreal networking could be worked out.

## Networking Valve | [check](https://github.com/ValveSoftware/GameNetworkingSockets)
Here the important things of valve networking could be worked out.

### Replicating | [check](https://wiki.unrealengine.com/Replication)
to be looked at



