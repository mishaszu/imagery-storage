# Imagery Storage
Imagery Storage

## Tech Stack:
- Axum
- Diesel
- async-graphql
- request

## Design:
user -> imagery -> lust
User makes a request to imagery, if requesting an image, imagery fetches it from Lust and sends it back to the user.

