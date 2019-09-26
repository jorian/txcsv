### TX history to CSV

For a given address, find all its transactions and record whether
- it was an incoming or outgoing transaction
- how much interest was claimed in the event of an outgoing transaction returning change

There are multiple scenarios:
1. The address is not found in the Vin array, which means money came in from someone else.
    1. we only look at the amount received and record that as Incoming
2. The address is found in the Vin array, and the address was not found in the Vout array. This means KMD was sent, but no change went back to the address
    1. we only look at the amount spent in the Vin array
3. The address is found in both the Vin and the Vout, which can be one of two cases:
    1. KMD was sent to an external address and change was returned to original address (ordinary payment)
        1. with interest
        2. without interest
    2. This was an interest claim