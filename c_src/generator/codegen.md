
# Generating C Code from `struct Note *`

notes can be converted to a number via a simple concat function on its struct fields

```pseudocode
// these arrays should be able to hold one of every note, thus achieving perfect hashing
// if this becomes unwieldingly large, a hash table would could this problem better
int size = 2 ^ (sizeof(struct Note) * 8)

Note regular[size] // array mapping id -> note
short inverse[size] // array mapping noteNumber -> id (inverse of regular)

total_notes = 0 // how many unique notes do we have in regular

fill inverse with -1

for (note in array) {
    noteNumber = toNumber(note)
    inverseIndex = inverse[noteNumber]
    if inverseIndex == -1 { // we haven't seen this note yet
        regular[total_notes] = note
        inverse[noteNumber] = total_notes
        total_notes += 1
    } 
    // by this point we know that we have our note in regular (without duplicates)
    writeNote(note)
}

// all of our song notes are written out
writeDefines(regular) 

```