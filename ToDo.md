# To Do

X build a lucidchart flow chart thing
X Start main.rs
X build some SQL insertion functions
X parse JMDict_b
X insert parsed data into database
X search functionality...
X make sure you have indexes on the things you need indexes for i.e. entries, kanji keb, reading reb, 
X materialised lookup table for entries
X play around with tauri
X edit entry_full and gloss_entry tables so that they only display one kanji, (by kanji priority)
X Underline on hover effect
X mouse "back" button matches return functionality
X Back end, conditional table query, based on character input, to serve to front end
X Update Home page japanese showings to show kanji + reading,,, ~~distinct on ent_seq~~
X create a "japanese entry" page
- restructure sense_eng table, linking table between sense and glosses
- let English entry page navigate to japanese entry page

- spruce up the look a little...
- import more data into database
    - concat strings for most of sense data probably
    X kanji has some info it needs like priority, whether it's outdated
    - readings have info as well
    - lots can be a string that can be parsed at runtime probably...
