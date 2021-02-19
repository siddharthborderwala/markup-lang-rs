# Markup Language

- Implement a simple markup language based on the concept of a finite state machine
- This is an instance where rust-lang's very powerful matching system stands out

## Rules for the language

- every character in between two # signs is a comment and will be stripped off
- every character in between two ^ signs will be converted to uppercase
- every character in between two _ signs will be converted to lowercase
