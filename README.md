# wooboo-v3-be
WooBoo server

## Tech Stack:
- Axum
- Diesel
- async-graphql
- request

## Design:
user -> wooboo -> lust
User makes a request to WooBoo, if requesting an image wooboo fetches it from Lust and sends it back to the user.

