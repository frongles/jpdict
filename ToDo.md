# To Do

X build a lucidchart flow chart thing
X Start main.rs
X build some SQL insertion functions
X parse JMDict_b
X insert parsed data into database
X search functionality...
X make sure you have indexes on the things you need indexes for i.e. entries, kanji keb, reading reb, 
X materialised lookup table for entries
- full text search functionality
- cli tool for interacting
- play around with tauri
- import more data into database
    - concat strings for most of sense data probably
    - kanji has some info it needs like priority, whether it's outdated
    - readings have info as well
    - lots can be a string that can be parsed at runtime probably...
- edit entry_full and gloss_entry tables so that they only display one kanji, (by kanji priority)
    - this will also cut down on db size significantly
